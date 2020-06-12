#!/bin/bash

gcloud builds submit --tag gcr.io/$PROJECT_ID/linkedin-oauth-endpoint --timeout=1h
