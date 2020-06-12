# linkedin oauth endpoint

🔬 THIS IS EXPERIMENTAL SOFTWARE NOT FOR USE IN PRODUCTION.  BE CAREFUL!  ⚠️ We intend no harm ⚠️

🤖 Callback must be exposed so that you can allow the app to share a status for your account 🤖

🙏 Check out the blog post about GOOGLE CLOUD RUN: [https://cprimozic.net/blog/rust-rocket-cloud-run/](https://cprimozic.net/blog/rust-rocket-cloud-run/)  🙏

## local dev 🛒

build it

```sh
docker build . -t linkedin-oauth-endpoint
```

run it

```sh
docker run -p 8020:8020 -p 8021:8021 linkedin-oauth-endpoint
```

## google cloud run

### Build

```sh
gcloud builds submit --tag gcr.io/PROJECT-ID/linkedin-oauth-endpoint
```


### Deploy

[deploy using google cloud run](https://cloud.google.com/run/docs/quickstarts/build-and-deploy?_ga=2.247509319.-199990648.1584658988#deploying_to) 

from the docs

```sh
gcloud builds submit --tag gcr.io/PROJECT-ID/linkedin-oauth-endpoint
```
