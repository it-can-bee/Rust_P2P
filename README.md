我的主页：[博客园](https://www.cnblogs.com/live-passion)

## 项目核心：
1、使用libp2p进行chat的msg传输、身份验证和ping工作


2、使用Swarm规定怎么在网络上传输字节数据、发送什么字节、发送的对象、PeerId身份和网络为止等


3、引入mDNS解析其他节点ip


## 运行
```cargo run```

## 运行结果
```我随机本地启动了三个终端，运行结果如下```
```
New PeerId is PeerId("12D3KooWSvNzGw5UJygyynJ5DtN3YqWjFcTtTVxhz8njVE2BGxjZ")
Listening on local address "/ip4/127.0.0.1/tcp/63420"
Listening on local address "/ip4/192.168.xx.xx/tcp/63420"
peers is discovered PeerId("12D3KooWMNZpMFm1Hi6y1kBQQcPUA1qW8KccsWeBTC7CdT725hBm") "/ip4/192.168.xx/tcp/63581"
peers is discovered PeerId("12D3KooWMNZpMFm1Hi6y1kBQQcPUA1qW8KccsWeBTC7CdT725hBm") "/ip4/127.0.0.1/tcp/63581"
peers is discovered PeerId("12D3KooWBASjZMNa93S4jyycLyThEfYumZWpSM8z2puLsad5aiKG") "/ip4/192.168.xx/tcp/63602"
peers is discovered PeerId("12D3KooWBASjZMNa93S4jyycLyThEfYumZWpSM8z2puLsad5aiKG") "/ip4/127.0.0.1/tcp/63602"


New PeerId is PeerId("12D3KooWMNZpMFm1Hi6y1kBQQcPUA1qW8KccsWeBTC7CdT725hBm")
Listening on local address "/ip4/127.0.0.1/tcp/63581"
Listening on local address "/ip4/192.168.xx.xx/tcp/63581"
peers is discovered PeerId("12D3KooWSvNzGw5UJygyynJ5DtN3YqWjFcTtTVxhz8njVE2BGxjZ") "/ip4/192.168.xx.xx/tcp/63420"
peers is discovered PeerId("12D3KooWSvNzGw5UJygyynJ5DtN3YqWjFcTtTVxhz8njVE2BGxjZ") "/ip4/127.0.0.1/tcp/63420"
peers is discovered PeerId("12D3KooWBASjZMNa93S4jyycLyThEfYumZWpSM8z2puLsad5aiKG") "/ip4/192.168.xx.xx/tcp/63602"
peers is discovered PeerId("12D3KooWBASjZMNa93S4jyycLyThEfYumZWpSM8z2puLsad5aiKG") "/ip4/127.0.0.1/tcp/63602"


New PeerId is PeerId("12D3KooWBASjZMNa93S4jyycLyThEfYumZWpSM8z2puLsad5aiKG")
Listening on local address "/ip4/127.0.0.1/tcp/63602"
Listening on local address "/ip4/192.168.xx.xx/tcp/63602"
peers is discovered PeerId("12D3KooWMNZpMFm1Hi6y1kBQQcPUA1qW8KccsWeBTC7CdT725hBm") "/ip4/192.168.xx.xx/tcp/63581"
peers is discovered PeerId("12D3KooWMNZpMFm1Hi6y1kBQQcPUA1qW8KccsWeBTC7CdT725hBm") "/ip4/127.0.0.1/tcp/63581"
peers is discovered PeerId("12D3KooWSvNzGw5UJygyynJ5DtN3YqWjFcTtTVxhz8njVE2BGxjZ") "/ip4/192.168.xx.xx/tcp/63420"
peers is discovered PeerId("12D3KooWSvNzGw5UJygyynJ5DtN3YqWjFcTtTVxhz8njVE2BGxjZ") "/ip4/127.0.0.1/tcp/63420"


``` 
