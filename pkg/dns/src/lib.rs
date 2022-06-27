use trust_dns_resolver::{config::ResolverOpts, error};
use trust_dns_resolver::proto::rr::rdata;
use trust_dns_resolver::TokioAsyncResolver;

#[derive(Clone)]
pub struct Resolver {
    dns: TokioAsyncResolver,
}

pub trait ConfigureResolver {
    fn configure_resolver(&self, _: &mut ResolverOpts);
}

#[derive(Debug, Clone, Error)]
#[error("invalid SRV record {:?}", self.0)]
struct InvalidSrv(rdata::SRV);

#[derive(Debug, Error)]
#[error("failed to resolve A record: {0}")]
struct ARecordError(#[from] error::ResolveError);

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
