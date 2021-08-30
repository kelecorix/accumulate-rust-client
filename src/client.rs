use jsonrpc_core_client::transports::local;
use jsonrpc_core::{Error, IoHandler, Result};
use jsonrpc_derive::rpc;

/// Accumulate API trait
#[rpc]
pub trait AccumulateRPC {
  /// Returns a protocol version
  #[rpc(name = "protocolVersion")]
  fn protocol_version(&self) -> Result<String>;

  /// accepts URL and returns a result as ADI object
  #[rpc(name = "adi", alias("callAsyncGetAccumulateIdentity"))]
  fn getADI(&self, u: &str) -> Result<ADI>;

   // accepts ADI structure with url and hash, signer, time and sginature
  ///  and returns a result as created ADI object
  #[rpc(name = "adi-create", alias("callAsyncCreateAccumulateIdentity"))]
  fn createADI(&self, u:ADI, s:Signer, t:UnixTime, sig: &str ) -> Result<ADI>;

  ///... other RPC methods go here

  /// Performs asynchronous operation
  #[rpc(name = "callAsync")]
  fn call(&self, u: string) -> FutureResult<String, Error>;
}

struct AccumulateRPCImpl;

impl AccumulateRPC for AccumulateRPCImpl {
  fn protocol_version(&self) -> Result<String> {
      Ok("version1".into())
  }

  fn getADI(&self, a: string) -> Result<ADI> {
      Ok(a)
  }

  fn call(&self, _: u64) -> FutureResult<String, Error> {
      future::ok("OK".to_owned())
  }
}

fn main() {
  let mut io = IoHandler::new();
  io.extend_with(RpcImpl.to_delegate());

  let fut = {
      let (client, server) = local::connect::<gen_client::Client, _, _>(io);
      // Make an actual call to API
      client.getADI("some-unique-url").map(|res| println!("adi = {}", res)).join(server)
  };
  fut.wait().unwrap();
}