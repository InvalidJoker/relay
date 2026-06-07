mod server;

fn main() {


    /*
    IDEA:
    - relay is the tcp proxy which runs under relay.invalidjoker.dev for example
    - it will forward the traffic to the actual server, which is running on localhost:8080 for example

    - for authetication (start hosting authentication) we request to the backend
    - we initiallaly will only support 1 relay connected


     Local Idea:
     add a relay.toml which stores everything so we can only do relay run --config and the cli will autonatilcy get everything and we can run relay

    Feature Ideas:
    - for http we add support for basic auth
    - persistent url
     */
    println!("Hello, world!");
}
