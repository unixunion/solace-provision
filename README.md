# Solace Provision

This solace-provision tool is written in Rust, which can process flat files to provision solace managed configuration items.

## Status

Fetch, Provision and Update, Shutdown, Enable, Delete for:

* VPNS
* Queues
* ACL Profiles
* Client Profiles
* Client Username

## Requirements

* Solace PubSub+ or SolOS-TR Appliance
* Solace's SEMPv2 service running in TLS mode

## Local Development

### Start Solace

    docker-compose up -d
    
### Manually enable TLS
    
Once the appliance is up, TLS must be enabled for SEMP. A development rootCA and localhost cert is available under [certs/](certs/), 
and you can follow Solace's documentation for setting it up with those or your own certs.

* Configure TLS for SEMP: https://docs.solace.com/Configuring-and-Managing/TLS-SSL-Service-Connections.htm#managing_tls_ssl_service_1762742558_317096
* Generating CA and Certs: https://gist.github.com/fntlnz/cf14feb5a46b2eda428e000157447309
* You can run the CA+Cert commands in /usr/sw/jail/certs on the router, access it with `docker-compose exec solace bash`
* Combine the server.crt and server.key into a single pem `cat localhost.crt localhost.key >>localhost.pem`
* enable TLS for SEMP as described in Solace Docs
* add the non-trusted rootca cert on client system and trust it, on the systems which will run the solace-provision tool. 
e.g: keychain import into System chain on mac + trust the cert.


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

NOTE, this tool needs built against a modified swagger-spec which you can find release branches for at https://github.com/unixunion/rust_solace_semp_client.git
The only change is that skip_deserialize for None types has been added to some structures that need it until swagger catches up.

If you want to link against a specific version of SEMPv2 API, you have some options:

    * use a release branch from https://github.com/unixunion/rust_solace_semp_client.git
    * request a backport release for your desired version from me.
    * use https://github.com/unixunion/rust_solace_semp_client.git to generate your own

Once you have decided on either of the above, you can edit Cargo.toml and modify the dep url/path for the rust_solace_semp_client.

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

solace-provision takes one config arg, and then a subcommand arg. 

See `solace-provision --help` for subcommands, each of which has an associated solace-provision {sub_command} --help

IMPORTANT: the msgVpnName key within the various yaml files is overridden at provision-time with the --message-vpn arg,
which is a mandatory arg for all operations except when creating a vpn, where it is an optional override.

Please remember that certain objects do reference each other, like client-usernames reference an client-profile and acl.

### VPN

#### Fetch VPN

    solace-provision --config examples/config.yaml --count 10 vpn --fetch --message-vpn "*"

#### Provision / Update VPN

    solace-provision --config examples/config.yaml vpn --file examples/vpn.yaml [--update]

#### Shutdown VPN

    solace-provision --config examples/config.yaml --message-vpn myvpn --shutdown --update
    
#### Enable VPN

    solace-provision --config examples/config.yaml --message-vpn myvpn --no-shutdown --update

#### Delete VPN

    solace-provision --config examples/config.yaml --message-vpn myvpn --delete

### Queue

#### Fetch Queue

    solace-provision --config examples/config.yaml --fetch-queue "*" --message-vpn myvpn [-n 10]

#### Provision Queue

    solace-provision --config examples/config.yaml --queue examples/queue.yaml [--update]

#### Shutdown Queue

    solace-provision --config examples/config.yaml --message-vpn myvpn --queue-name myqueue --shutdown --update
    
#### Enable Queue

    solace-provision --config examples/config.yaml --message-vpn myvpn --queue-name myqueue --no-shutdown --update

### ACL

#### Fetch ACL

    solace-provision --config examples/config.yaml --fetch-acl-profile "*" --message-vpn myvpn

#### Provision ACL

    solace-provision --config examples/config.yaml --acl examples/acl.yaml --message-vpn myvpn [--update]
    
### Client Profile

#### Fetch Client-Profile

    solace-provision --config examples/config.yaml --fetch-client-profile "*" --message-vpn myvpn

#### Provision Client-Profile

    solace-provision --config examples/config.yaml --client-profile examples/client-profile.yaml --message-vpn myvpn [--update]

### Client-Username
    
#### Fetch Client Username

    solace-provision --config examples/config.yaml --fetch-client-username "*" --message-vpn myvpn -n 10  
    
#### Provision Client-Username

    solace-provision --config examples/config.yaml --client-username examples/client-username.yaml --message-vpn myvpn [--update]

## References

https://docs.solace.com/API-Developer-Online-Ref-Documentation/swagger-ui/index.html
https://github.com/swagger-api/swagger-codegen/blob/master/samples/client/petstore/rust/examples/client.rs

