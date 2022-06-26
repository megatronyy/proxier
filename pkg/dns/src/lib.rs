use trust_dns_resolver::config::ResolverOpts;
use trust_dns_resolver::TokioAsyncResolver;

#[derive(Clone)]
pub struct Resolver{
    dns: TokioAsyncResolver,
}

pub trait ConfigureResolver {
    fn configure_resolver(&self, _: &mut ResolverOpts);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
