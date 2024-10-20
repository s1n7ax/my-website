# My Website

This is [https://s1n7ax.com](https://s1n7ax.com) built with Rust 🦀

![Screenshot PC 1](https://github.com/s1n7ax/my-website-ssr/assets/18459807/ebbe2fa6-03fc-4fa9-9e3e-53fb9014682b)
![Screenshot Mobile 1](https://github.com/s1n7ax/my-website-ssr/assets/18459807/1ff22851-b152-414d-8603-3fb697086d3d)

## Development

- Clone the project
- Prepare the dev environment
  - Using direnv (recommended)
    - Allow the direnv to initiate the environment using `direnv allow`
  - Using Devcontainer
    - Run the dev container `devcontainer up --workspace-folder .`
    - Open up the shell `devcontainer exec --workspace-folder . bash`
- Install node dependencies `yarn install`
- Run tailwind watch process `yarn dev`
- Open the browser `http://localhost:3000`

## Deployment

Deployment is done using docker images.

Github workflow in this project, builds a new image and push it to docker hub.
For the deployment, SSH should be configured in the server.

Following secrets should be registered in the Github Actions

- `DOCKERHUB_USERNAME` - Name of the docker hub user
- `DOCKERHUB_TOKEN` - Access token
- `DO_IP` - IP of the server
- `DO_USERNAME` - Name of the user in the server
- `DO_SSH_PRIVATE_KEY` - SSH Private key

## SSL & TLS

Using the `certbot` client, we can request a new certificate.

```shell
docker compose up certbot
```

Let's encrypt certificate will be outdated in 90 days. So another request has
to be made to update. This can be automated using a cron job

### Automated SSL renewal

[This documentation](https://eff-certbot.readthedocs.io/en/latest/using.html#setting-up-automated-renewal)
shows how to add a crone job to automatically renew the certificate
