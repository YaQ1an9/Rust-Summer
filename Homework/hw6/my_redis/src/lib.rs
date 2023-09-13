#![feature(impl_trait_in_assoc_type)]

use std::{collections::HashMap, sync::Mutex};

use pilota::FastStr;
use volo_gen::volo::redis::{RedisCommand, GetItemResponse, GetItemRequest};

pub struct S {
    pub map: Mutex<HashMap<String, String>>,
}
pub const DEFAULT_ADDR: &str = "127.0.0.1:8080";

#[volo::async_trait]
impl volo_gen::volo::redis::ItemService for S {
    async fn get_item(
        &self,
        _req: volo_gen::volo::redis::GetItemRequest,
    ) -> ::core::result::Result<volo_gen::volo::redis::GetItemResponse, ::volo_thrift::AnyhowError>
    {
        match _req.cmd {
            RedisCommand::Get => {
                if let Some(arg) = _req.args {
                    if arg.len() != 1 {
                        Ok(GetItemResponse { 
                            ok: false, 
                            data: Some(FastStr::from(format!(
                                "Args Error! Expected 1, got {}", 
                                arg.len()
                            )))
                        }) 
                    } else {
                        if let Some(value) = self.map.lock().unwrap().get(&arg[0].to_string()) {
                            Ok(GetItemResponse { 
                                ok: true, 
                                data: Some(FastStr::from(value.to_string()))
                            })
                        } else {
                            Ok(GetItemResponse { 
                                ok: false, 
                                data: Some(FastStr::from("Error"))
                            })
                        }
                    }
                } else {
                    Ok(GetItemResponse { 
                        ok: false, 
                        data: Some(FastStr::from("Args Error!"))
                    })
                }
            }
            RedisCommand::Set => {
                if let Some(arg) = _req.args {
                    if arg.len() < 2 {
                        Ok(GetItemResponse { 
                            ok: false,
                            data: Some(FastStr::from(format!(
                                "Args Error! Expected 2, got {}", 
                                arg.len()
                            ))) 
                        })
                    } else {
                        let (key, value) = (&arg[0], &arg[1]);
                        if self.map.lock().unwrap().insert(key.to_string(), value.to_string()).is_some() {
                            Ok(GetItemResponse { 
                                ok: true,
                                data: Some(FastStr::from("Ok,Updated!")) 
                            })
                        } else {
                            Ok(GetItemResponse { 
                                ok: true,
                                data: Some(FastStr::from("Ok, Insert Success!")) 
                            })
                        }
                    }
                } else {
                    Ok(GetItemResponse { 
                        ok: false,
                        data: Some(FastStr::from("Args Error!")) 
                    })
                }
            }
            RedisCommand::Del => {
                if let Some(arg) = _req.args {
                    if arg.len() < 1 {
                        Ok(GetItemResponse { 
                            ok: false,
                            data: Some(FastStr::from(format!(
                                "Args Error! Expected 1, got {}", 
                                arg.len()
                            ))) 
                        })
                    } else {
                        let mut count = 0;
                        for key in arg {
                            count += self.map.try_lock().unwrap().remove(&(key.to_string())).is_some() as i32;
                        }
                        Ok(GetItemResponse { 
                            ok: true,
                            data: Some(FastStr::from(format!("{}", count))) 
                        })
                    }
                } else {
                    Ok(GetItemResponse { 
                        ok: false,
                        data: Some(FastStr::from("Args Error!")) 
                    })
                }
            }
            RedisCommand::Ping => {
                if let Some(arg) = _req.args {
                    if arg.len() == 0{
                        return Ok(GetItemResponse { 
                            ok: true,
                            data: Some(FastStr::from("PONG"))
                        })
                    } else {
                        Ok(GetItemResponse { 
                            ok: true,
                            data: Some(FastStr::from(arg.join(" "))) 
                        })
                    }
                } else {
                    Ok(GetItemResponse { 
                        ok: true,
                        data: Some(FastStr::from("PONG")) 
                    })
                }
            }
            RedisCommand::Publish => { 
                Ok(GetItemResponse { 
                    ok: false,
                    data: Some(FastStr::from("Command Not impl!")) 
                })
            }
            RedisCommand::Subscribe => {
                Ok(GetItemResponse { 
                    ok: false,
                    data: Some(FastStr::from("Command Not impl!")) 
                })
            }
            _ => {
                Ok(GetItemResponse { 
                    ok: false,
                    data: Some(FastStr::from("Command Not Found!")) 
                })
            }
        }
    }
}

pub struct LogLayer;

impl<S> volo::Layer<S> for LogLayer {
    type Service = LogService<S>;

    fn layer(self, inner: S) -> Self::Service {
        LogService(inner)
    }
}


#[derive(Clone)]
pub struct LogService<S>(S);

#[volo::service]
impl<Cx, Req, S> volo::Service<Cx, Req> for LogService<S>
where
    Req: std::fmt::Debug + Send + 'static,
    S: Send + 'static + volo::Service<Cx, Req> + Sync,
    S::Response: std::fmt::Debug,
    S::Error: std::fmt::Debug,
    Cx: Send + 'static,
{
    async fn call(&self, cx: &mut Cx, req: Req) -> Result<S::Response, S::Error> {
        let now = std::time::Instant::now();
        tracing::debug!("Received request {:?}", &req);
        let resp = self.0.call(cx, req).await;
        tracing::debug!("Sent response {:?}", &resp);
        tracing::info!("Request took {}ms", now.elapsed().as_millis());
        resp
    }
}
pub struct FilterLayer;

impl<S> volo::Layer<S> for FilterLayer {
    type Service = FilterService<S>;

    fn layer(self, inner: S) -> Self::Service {
        FilterService(inner)
    }
}
#[derive(Clone)]
pub struct FilterService<S>(S);
#[volo::service]
impl<Cx, Req, S> volo::Service<Cx, Req> for FilterService<S>
where
    Req: std::fmt::Debug + Send + 'static,
    S: Send + 'static + volo::Service<Cx, Req> + Sync,
    S::Response: std::fmt::Debug,
    S::Error: std::fmt::Debug,
        anyhow::Error: Into<S::Error>,
    Cx: Send + 'static,
{
    async fn call(&self, cx: &mut Cx, req: Req) -> Result<S::Response, S::Error> {
        let info = format!("{:?}", req);
        if info.contains("csj") {
            return Err(anyhow::anyhow!("[csj] is not allowed").into());
        }
        self.0.call(cx, req).await
    }
}
