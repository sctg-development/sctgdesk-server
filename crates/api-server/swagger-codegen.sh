#!/bin/bash
response=$(curl -s -o /dev/null -w "%{http_code}" http://127.0.0.1:21114/openapi.json)

if [ "$response" -eq 200 ]; then
    echo "Success! Server is up and running."
    # Continue
    curl http://127.0.0.1:21114/openapi.json >webconsole/src/api/openapi.json &&
        docker run --rm -v ${PWD}:/local swaggerapi/swagger-codegen-cli-v3 generate -i file:///local/webconsole/src/api/openapi.json -l typescript-axios -o /local/webconsole/src/api &&
        sed -ibak 's/accessToken/access_token/' webconsole/src/api/models/login-reply.ts &&
        rm webconsole/src/api/models/login-reply.tsbak
else
    echo "Error: Server is not responding. you must start sctgdesk-api-server first."
    exit 1
fi
