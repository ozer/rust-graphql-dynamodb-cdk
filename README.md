# Rust GraphQL API

## Deploy to AWS Fargate using CDK

### Dev setup for Rust GraphQL API

- Make sure you have IAM User with `DynamoDB` privileges.
- Make sure you have a DynamoDB Table created `CoffeeShop` or you might wanna change it.
- Keep in mind keep table names consistent as you deploy it via CDK.

```sh
cp .env.dev .env
```

- Fill your `.env` file with credentials.

- `cargo run`

---

### Deploying AWS Fargate using CDK via GitHub Actions

- Make sure you define below credentials in your `GitHub Actions` settings.

#### For CDK Deployment

- AWS_ACCESS_KEY_ID
- AWS_SECRET_ACCESS_KEY
- AWS_ACCOUNT

#### For Rust API to talk DynamoDB

- API_AWS_ACCESS_KEY_ID
- API_AWS_SECRET_ACCESS_KEY

---

- Unfortunately, I haven't figured it out sufficient IAM permissions for CDK stuff so, make sure your IAM User has administrator privileges.
- So, `AWS_ACCESS_KEY_ID` and `AWS_SECRET_ACCESS_KEY` should belong to an IAM User with admin privileges.
- `API_AWS_ACCESS_KEY_ID` and `API_AWS_SECRET_ACCESS_KEY` should belong to an IAM User with sufficient `DynamoDB` privileges.
