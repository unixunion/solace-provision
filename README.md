# Solace Provision

This solace-provision tool is written in Rust, which can process flat files with all possible SEMP configurables.

![Alt text](screenshot.png?raw=true "Title")

## Status

Fetch, Provision and Update for:

* VPNS
* Queues
* ACL Profiles
* Client Profiles
* Client Username

Todo:
* override VPN name allowing portability
* shutdown apply mode
    
## Solace Bug

Due to a bug in how Solace handles `Threshold` objects, setValue and clearValue cannot be used. Please use *percent*. 
That is why you will see *value* related thresholds commented out. This project depends on a modified Swagger spec until
Solace sorts the issue. The root cause is that when we create a instance of MsgVpn using the Swagger Spec, EventThreshold has 
all 4 thresholds set, but the appliance / vmr exceptions when you submit the Threshold object with all 4 keys present, 
regardless of the key value.

## Requirements

* Solace PubSub+ or SolOS-TR Appliance
* Solace's SEMP service running in TLS mode

## Local Development

### Start Solace

    docker-compose up -d
    
### Manually enabling TLS
    
Once the appliance is up, TLS must be enabled for SEMP. A rootCA and localhost cert is available under [certs/](certs/), 
or you can follow Solace's documentation for setting it up.

* Configure TLS for SEMP: https://docs.solace.com/Configuring-and-Managing/TLS-SSL-Service-Connections.htm#managing_tls_ssl_service_1762742558_317096
* Generating CA and Certs: https://gist.github.com/fntlnz/cf14feb5a46b2eda428e000157447309
* You can run the CA+Cert commands in /usr/sw/jail/certs on the router, access it with `docker-compose exec solace bash`
* Combine the server.crt and server.key into a single pem `cat localhost.crt localhost.key >>localhost.pem`
* enable TLS for SEMP as described in Solace Docs
* add rootca cert on client host system which will run this code. e.g: keychain import into System chain on mac.


```bash
77528f005592> enable 
77528f005592# configure 
77528f005592(configure)# service semp shutdown
77528f005592(configure)# authentication
77528f005592(configure/authentication)# certificate-authority rootCa.crt
77528f005592(configure)# ssl 
77528f005592(configure/ssl)# server-certificate localhost.pem
77528f005592(configure)# service semp listen-port 8080 ssl
77528f005592(configure)# service semp no shutdown ssl

```


Testing TLS:

    curl -k --cacert ./certs/rootCa.crt https://localhost:8080/SEMP/v2/config 

# Compiling

    cargo build --release

# Provision / Update VPN

`solace-provision` can <i>create</i> or <i>update</i> existing VPN's. Running without `--update` assumes "create" behaviour. 
See `solace-provision --help` for more info.

## Configuring API Client

See [examples/config.yaml](examples/config.yaml) for appliance connection properties. Pass the config file with: `--config examples/config.yaml`

## VPN Provision Config

The vpn.yaml example contains all the possible keys and values settable. 
See [vpn.yaml](examples/vpn.yaml)
    
## Queue Provision Config

The queue.yaml example contains all the possible keys and values settable. 
See [queue.yaml](examples/queue.yaml)
    
## Provisioning

IMPORTANT: the msgVpnName key within the various yaml files is overridden at provision-time with the --message-vpn arg,
which is "default" by default. you MUST specify the VPN to perform the provision in for ALL items except VPN.

That said, please remember that certain objects need to reference each other, like client-usernames reference an client-profile and acl.


### Shutdown VPN

    solace-provision --config examples/config.yaml --message-vpn myvpn --shutdown --update

### Provision VPN

    solace-provision --config examples/config.yaml --vpn examples/vpn.yaml [--update]

### Provision Queue

    solace-provision --config examples/config.yaml --queue examples/queue.yaml [--update]
    
### Provision ACL

    solace-provision --config examples/config.yaml --acl examples/acl.yaml [--update]
    
### Provision Client Profile

    solace-provision --config examples/config.yaml --client-profile examples/client-profile.yaml [--update]
    
### Provision Client Username

    solace-provision --config examples/config.yaml --client-username examples/client-username.yaml [--update]


### Fetch VPN

    solace-provision --config examples/config.yaml --fetch-acl-profile "*" -n 10

### Fetch Queue

    solace-provision --config examples/config.yaml --fetch-vpn "*" -n 10

## Fetch ACL

    solace-provision --config examples/config.yaml --fetch-queue "*" -n 10   

## Fetch Client Profile

    solace-provision --config examples/config.yaml --fetch-client-profile "*" -n 10   
    
## Fetch Client Username

    solace-provision --config examples/config.yaml --fetch-client-username "*" -n 10   



## References

https://docs.solace.com/API-Developer-Online-Ref-Documentation/swagger-ui/index.html
https://github.com/swagger-api/swagger-codegen/blob/master/samples/client/petstore/rust/examples/client.rs

