# Solace Provision

solace-provision is a tool which reads flat files to provision solace hardware appliances and software brokers.

## Project Status

Currently in *beta*

Capabilities:

    * Fetch + Save to disk
    * Provision
    * Update
    * Shutdown
    * Enable
    * Delete

Objects that can be Provisioned, Updated and Downloaded

    * VPN
    * Queue
    * ACL Profile
    * Client Profile
    * Client Username
    
This tool is subject to [SEMPv2 limitations](https://docs.solace.com/SEMP/SEMP-API-Versions.htm#SEMPv2).

## Requirements

Solace PubSub 9.X or SolOS-TR Appliance 8.x with <b>TLS</b> enabled for the SEMP service. Without SEMP, some configurations
will throw a exception due to plain-text transmission of "sensitive" information.

## Compiling Requirements

rust or docker

# Usage

solace-provision takes in YAML files, and some command line args in order to provision Solace managed objects.
see `solace-provision --help` for details on each <i>subcommand</i>.

Example:

    solace-provision --config config.yaml vpn --message-vpn myvpn --file vpnspec.yaml --shutdown --update --no-shutdown

## Configuring and Spec files


### Configuring API Client

Example Config:
```yaml
username: admin
password: admin
host: https://localhost:8080/SEMP/v2/config
```

See [examples/config.yaml](examples/config.yaml) 

### Spec Files

solace-provison uses YAML files to configure Solace managed objects, all keys and possible values can be found within the 
OpenAPI generated api. I provide a build-tool for generating the API, see: [rust_solace_semp_client](https://github.com/unixunion/rust_solace_semp_client.git)

Examples Provision Files:

* [vpn.yaml](examples/vpn.yaml) 
* [queue1.yaml](examples/queue1.yaml)
* [acl.yaml](examples/acl.yaml)
* [client-profile.yaml](/examples/client-profile.yaml)
* [client-username.yaml](/examples/client-username.yaml)

## Provisioning

When provisioning, consider the order of dependencies between items e.g: 

`VPN -> ACL -> CLIENT-PROFILE -> CLIENT-USERNAME -> QUEUE`

IMPORTANT: the <i>msgVpnName</i> key within the various yaml files is overridden at provision-time with the `--message-vpn` arg,
which is a mandatory arg for *all* operations.

Executable quick overview:

```bash
solace-provision --config {CLIENT_CONFIG} \
                [--output {FETCH_OUTDIR}] \
                vpn|queue|acl-profile|client-profile|client-username \
                --message-vpn {VPN_NAME} \
                [--file {ITEM_YAML}] \
                [--queue|--acl-profile|--client-profile|--client-username] {ITEM_NAME}  \
                [--update] \
                [--shutdown] \
                [--no-shutdown] \
                [--fetch]
```

### Order of Operation

solace-provision performs all operations the order depicted below, any operation that fails will result in the process terminating.

![schematic](schematic.png)

### Error Prevention

Due to limitations in the SEMPv2 spec, many attributes are passed as string directly to the appliance, this means you could pass 
invalid / out-of-spec values, resulting in the provision aborting. Therefore you should ensure your strategy involves applying changes
to test vpns / appliances before attempting production changes. As the diagram above shows, if you attempt to update a vpn
while shutting it down, if the update fails you will be left with a vpn in shutdown mode as no further operations will be performed.

Tip, the `--message-vpn` arg will override the VPN a object is being made / updated in, so use it to apply your change to a sandbox 
VPN before targeting the change at a in-use one.

### Logging

Logging is configured with the `RUST_LOG` environment variable, set to `[warn|error|info|debug]`. Example:

    RUST_LOG=solace_provision ...
    RUST_LOG=solace_provision=error solace-provision ...

### Running

solace-provision takes args both within the subcommand scope and outside of it. Outside subcojmand args are:

    * --config file MANDATORY
    * --output OPTIONAL: directory for "fetch" operations 
    * --count n OPTIONAL: items per "fetch", default=10


### VPN Subcommand


#### Fetch VPN

    solace-provision --config examples/config.yaml [--count 10] vpn --fetch --message-vpn "*"

#### Fetch VPN and Write to output dir:

    solace-provision --config examples/config.yaml [--output ./out_dir] [--count 10] vpn --fetch --message-vpn "*"    

#### Provision / Update VPN

    solace-provision --config examples/config.yaml vpn --file examples/vpn.yaml [--update]

#### Shutdown VPN

    solace-provision --config examples/config.yaml vpn --message-vpn myvpn --shutdown --update
    
#### Enable VPN

    solace-provision --config examples/config.yaml vpn --message-vpn myvpn --no-shutdown --update

#### Delete VPN

    solace-provision --config examples/config.yaml vpn --message-vpn myvpn --delete

### Queue Subcommand

#### Fetch Queue

    solace-provision --config examples/config.yaml [--output ./out_dir] [--count 10] queue --fetch-queue "*" --message-vpn myvpn

#### Provision Queue

    solace-provision --config examples/config.yaml queue --queue examples/queue.yaml [--update]

#### Shutdown Queue

    solace-provision --config examples/config.yaml queue --message-vpn myvpn --queue-name myqueue --shutdown --update
    
#### Enable Queue

    solace-provision --config examples/config.yaml queue --message-vpn myvpn --queue-name myqueue --no-shutdown --update

### ACL Subcommand

#### Fetch ACL

    solace-provision --config examples/config.yaml [--output tmp] [--count 10] acl-profile --fetch --acl-profile "*" --message-vpn myvpn

#### Provision ACL

    solace-provision --config examples/config.yaml acl-profile --file examples/acl.yaml --message-vpn myvpn [--update]
    
### Client Profile

#### Fetch Client-Profile

    solace-provision --config examples/config.yaml [--output tmp] [--count 10] client-profile --fetch --client-profile "*" --message-vpn myvpn

#### Provision Client-Profile

    solace-provision --config examples/config.yaml client-profile --file examples/client-profile.yaml --message-vpn myvpn [--update]

### Client-Username
    
#### Fetch Client Username

    solace-provision --config examples/config.yaml [--output tmp] [--count 10]  client-username --client-username "*" --message-vpn myvpn 
    
#### Provision Client-Username

    solace-provision --config examples/config.yaml client-username --file examples/client-username.yaml --message-vpn myvpn [--update]


## Compiling From Source

Consider what version of appliance you run before compiling, as you should compile `solace-provision` with the lowest common
appliance version you have in your cluster. see: [cargo.toml](/cargo.toml)

If you dont find a supported version in the [rust_solace_semp_client](https://github.com/unixunion/rust_solace_semp_client.git)
repo, you can make a request for one, or you can make your own using [solace_semp_client](https://github.com/unixunion/solace_semp_client.git).

Then simply point the dependency in cargo.toml to the filepath or git repo + branch where you have your generated OpenAPI 
classes.

Using cargo ( produces arch binary)

```bash
cargo build --release
```

Using Docker ( produces Linux binary )

```bash
docker run -v `pwd`:/src rust:1.33 /src/mkrelease.sh
```

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

If you want to link against a specific version of SEMPv2 API, you have some options:

    * use a release branch from https://github.com/unixunion/rust_solace_semp_client.git
    * request a backport release for your desired version from me.
    * use https://github.com/unixunion/rust_solace_semp_client.git to generate your own

Once you have decided on either of the above, you can edit Cargo.toml and modify the dep url/path for the rust_solace_semp_client.

## References

https://docs.solace.com/API-Developer-Online-Ref-Documentation/swagger-ui/index.html
https://github.com/swagger-api/swagger-codegen/blob/master/samples/client/petstore/rust/examples/client.rs

