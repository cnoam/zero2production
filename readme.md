# Deploying to DigitalOcean

Deployment is covered in chapter 5.

The steps:
1. create a token
2. remove the old token `doctl auth remove --context zero2prod_ctx`
3. install the new token `doctl auth init --context zero2prod_ctx`
4. deploy `doctl apps create --spec spec.yaml`

**Remember to delete the app to save money**
