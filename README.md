# docker-logs-transfer
Reformat [docker JSON log](https://docs.docker.com/config/containers/logging/json-file/) to read friendly

# How to use

Use the following command to replace existing log:
```
./docker-logs-transfer -p your.log
```

Or the following command to output it into a new file:
```
./docker-logs-transfer -p your.log -o out.log
```

Use `./docker-logs-transfer -h` to show more options
