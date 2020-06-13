#!/usr/bin/env node
import 'source-map-support/register';
import * as cdk from '@aws-cdk/core';
import { CdkStack } from '../lib/cdk-stack';

const app = new cdk.App();
new CdkStack(app, 'CdkStack', {
    env: {
        account: process.env.AWS_ACCOUNT,
        region: 'eu-central-1'
    }
});
