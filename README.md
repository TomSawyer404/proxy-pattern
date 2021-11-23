# Proxy 模式

**代理**是一种结构型设计模式， 让你能提供真实服务对象的替代品给客户端使用。 代理接收客户端的请求并进行一些处理 （访问控制和缓存等）， 然后再将请求传递给服务对象。 

代理对象拥有和服务对象相同的接口， 这使得当其被传递给客户端时可与真实对象互换。

代理模式的结构如下图：

![proxy](https://refactoringguru.cn/images/patterns/diagrams/proxy/structure.png)

1. `ServiceInterface`声明了服务接口。代理必须遵循该接口才能伪装成服务对象；

2. `Service`类提供了一些实用的业务逻辑；
3. `Proxy`类包含一个执行服务对象的引用成员变量。代理完成其任务（如延迟初始化、记录日志、访问控制和缓存等）后会将请求传递给服务对象。通常情况下，代理会对其服务器对象的整个生命周期进行管理；
4. `Client`能通过同一接口与服务或代理进行交互，所以你可以在一切需要服务对象的代码中使用代理；



# 实战

Nginx 这样的 Web Server 可以充当 Application Server 的代理：

- 提供了对 Application Server 的受控访问权限；
- 可限制速度；
- 可缓存请求；



# 参考资料

- [适配器模式-Wikipedia](https://en.wikipedia.org/wiki/Proxy_pattern)
- [lpxxn的示例代码](https://github.com/lpxxn/rust-design-pattern/blob/master/structural/proxy.rs)
- [yukihir0的示例代码](https://github.com/yukihir0/rust_design_pattern/blob/master/proxy/src/main.rs)
- [适配器模式](https://refactoringguru.cn/design-patterns/proxy)

---
