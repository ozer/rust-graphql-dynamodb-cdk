import * as cdk from "@aws-cdk/core";
import { Vpc, SubnetType } from "@aws-cdk/aws-ec2";
import {
  Cluster,
  ContainerImage,
  FargateTaskDefinition,
  FargateService,
  AwsLogDriver,
} from "@aws-cdk/aws-ecs";
import { ApplicationLoadBalancer } from "@aws-cdk/aws-elasticloadbalancingv2";

export class CdkStack extends cdk.Stack {
  constructor(scope: cdk.Construct, id: string, props?: cdk.StackProps) {
    super(scope, id, props);

    // The code that defines your stack goes here
    const vpc = new Vpc(this, "CoffeeShopVPC", {
      cidr: "11.180.0.0/16",
      maxAzs: 2,
      natGateways: 0,
      vpnGateway: false,
      subnetConfiguration: [
        {
          name: "coffee-shop-subnet-1",
          subnetType: SubnetType.PUBLIC,
        },
      ],
    });

    const cluster = new Cluster(this, "CoffeeShopCluster", { vpc });

    const image = ContainerImage.fromAsset("../");

    const taskDefinition = new FargateTaskDefinition(
      this,
      "CoffeeShopTaskDefinition",
      {
        memoryLimitMiB: 512,
        cpu: 256,
      }
    );

    const container = taskDefinition.addContainer("CoffeeShopAPIContainer", {
      image,
      environment: {
        STAGE: "prod",
        AWS_ACCESS_KEY_ID: process.env?.AWS_ACCESS_KEY_ID || ":(",
        AWS_SECRET_ACCESS_KEY: process.env?.AWS_SECRET_ACCESS_KEY || ":(",
      },
      cpu: 128,
      memoryLimitMiB: 256,
      logging: new AwsLogDriver({ streamPrefix: "coffee-shop" }),
    });

    container.addPortMappings({ containerPort: 8080 });

    const service = new FargateService(this, "CoffeeShopService", {
      cluster,
      taskDefinition,
      desiredCount: 1,
      assignPublicIp: true,
    });

    const alb = new ApplicationLoadBalancer(this, "CoffeeShopALB", {
      vpc,
      internetFacing: true,
    });

    const listener = alb.addListener("CoffeeShopListener", { port: 80 });

    const targetGroup = listener.addTargets("CoffeeShopTG1", {
      port: 80,
      healthCheck: {
        interval: cdk.Duration.seconds(60),
        timeout: cdk.Duration.seconds(7),
        port: "8080",
        path: "/health",
      },
      targets: [
        service.loadBalancerTarget({
          containerName: "CoffeeShopAPIContainer",
          containerPort: 8080,
        }),
      ],
    });
  }
}
