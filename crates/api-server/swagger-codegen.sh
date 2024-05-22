#!/bin/bash
response=$(curl -s -o /dev/null -w "%{http_code}" http://127.0.0.1:21114/openapi.json)

if [ "$response" -eq 200 ]; then
    echo "Success! Server is up and running."
    # Continue
    curl http://127.0.0.1:21114/openapi.json >webconsole/src/api/openapi.json &&
        docker run --rm -v ${PWD}:/local swaggerapi/swagger-codegen-cli-v3 generate -i file:///local/webconsole/src/api/openapi.json --additional-properties modelPropertyNaming=snake_case -l typescript-axios -o /local/webconsole/src/api &&
        sed -ibak -e 's/confirm_password/\"confirm-password\"/' webconsole/src/api/models/add-user-request.ts &&
        sed -ibak -e 's/confirm_password/\"confirm-password\"/' webconsole/src/api/models/update-user-request.ts &&
        sed -ibak -e 's/device_info/\"deviceInfo\"/' webconsole/src/api/models/oidc-auth-request.ts &&
        rm webconsole/src/api/models/*tsbak
else
    echo "Error: Server is not responding. you must start sctgdesk-api-server first."
    exit 1
fi
