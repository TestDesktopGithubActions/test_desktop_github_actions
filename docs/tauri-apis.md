# Tauri-apis

## Apis

### Tauri交互参数

#### 成功响应

| Parameter      |     Type |   Parameter Description   |
| :-------- | :--------| :------ |
| code|  Int|  200=成功 |
| message|  String|  提示描述|
| result|  map or array |  k/v or array or null|

#### 错误响应

| Parameter      |     Type |   Parameter Description   |
| :-------- | :--------| :------ |
| code|  Int|  200=成功 |
| message|  String|  提示描述|
| result|  map or array |  k/v or array or null|

#### Code说明

| Parameter      |     Type |   Parameter Description   |
| :-------- | :--------| :------ |
| 200 |  Int|  成功 |
| 201 |  Int|  提示或警告|
| 203 |  Int|  有误|
| 204 |  Int|  违规操作|
| 404 |  Int|  拒绝访问|
| 500 |  Int|  系统错误|

#### JWT Code说明

| Parameter      |     Type |   Parameter Description   |
| :-------- | :--------| :------ |
| 2001 |  Int|  失效 |
| 2002 |  Int|  过期|
| 2008 |  Int|  登录|

***

### 公共

#### 登录: login

> Requests and Parameter Description

| Parameter | Type   | Must | Parameter Description |
| :-------- | :----- | :--- | :-------------------- |
| email     | String | 是   | 邮箱                  |
| password  | String | 是   | 密码                  |

>Response and Parameters in data

| Parameter    | Type   | Parameter Description                 |
| :----------- | :----- | :------------------------------------ |
| token        | String | 登陆凭证/5个小时有效                  |
| email        | String | 邮箱                                  |
| coin_balance | f64    | gb余额                                |
| balance      | f64    | 余额                                  |
| comm_balance | f64    | 佣金余额                              |
| code         | int    | 客户编码/邀请码                       |
| is_card      | int    | 是否购买套餐 0/未购买,1/购买,2/已过期 |
| card_id      | string | 套餐id                                |
| start_at     | string | 套餐开始时间                          |
| end_at       | string | 套餐结束时间                          |
| surplus_day  | int    | 剩余天数                              |
| exp          | int    | token过期时间,5个小时                 |
| is_usdt      | int    | 1/no,2/yes                            |
| usdt_addr    | string | usdt 地址                             |

***



#### 登出: logout

> Requests and Parameter Description

| Parameter | Type   | Must | Parameter Description |
| :-------- | :----- | :--- | :-------------------- |
| token     | String | 是   | Token                 |

>Response and Parameters in data

| Parameter | Type   | Parameter Description |
| :-------- | :----- | :-------------------- |
| success   | string | success               |

***



#### 注册: register

> Requests and Parameter Description

| Parameter         | Type   | Must | Parameter Description |
| :---------------- | :----- | :--- | :-------------------- |
| email             | String | 是   | 邮箱                  |
| passed            | String | 是   | 密码                  |
| repeated_password | String | 是   | 再次输入密码          |

>Response and Parameters in data

| Parameter    | Type   | Parameter Description |
| :----------- | :----- | :-------------------- |
| account_code | String | 注册返回code          |

***



#### 绑定设备: bind_device

> Requests and Parameter Description
>
> > not Parameter


>Response and Parameters in data

| Parameter      |     Type |      Parameter Description   |
| :-------- | :--------| :------ |
| guid|  String| 设备guid|
| def|  String| 设备定义 |
| proof|  String| 设备uuid |
| code|  int| 用户推广码|
| is_check|  int| 是否注册 1/没有注册,2/注册|
| created_at|  string| 创建时间|

***

#### 激活账号: activating

> Requests and Parameter Description

| Parameter      |     Type |     Must |   Parameter Description   |
| :-------- | :--------| :------ | :------ |
| account_code | int |  是| 注册返回code |
| code|  int|   是| 邮箱验证码|
> Response and Parameters in data
>> to login Parameters

***

#### 更新token: account_update_token

> 
>Requests and Parameter Description

| Parameter | Type   | Must | Parameter Description |
| :-------- | :----- | :--- | :-------------------- |
| token     | String | 是   | token                 |

> 
>Response and Parameters to in login Parameters

***

#### 日志上传: upload_log

> Requests and Parameter Description

| Parameter | Type   | Must | Parameter Description |
| :-------- | :----- | :--- | :-------------------- |
| token     | String | 是   | token                 |
| email     | String | 是   | 邮箱                  |
| name      | String | 是   | 文件名称              |
| url       | String | 是   | 文件路径              |

>Response and Parameters in data

| Parameter | Type   | Parameter Description  |
| :-------- | :----- | :--------------------- |
| url       | String | 服务器存储日志文件位置 |

***

#### 设置语言: set_language

> Requests and Parameter Description

| Parameter | Type   | Must | Parameter Description |
| :-------- | :----- | :--- | :-------------------- |
| language  | String | 是   | 语言                  |

>Response and Parameters in data
>
>no data

***







### 节点

#### 节点列表: node_list

> Requests and Parameter Description

| Parameter | Type   | Must | Parameter Description |
| :-------- | :----- | :--- | :-------------------- |
| token     | String | 是   | token                 |

> Response and Parameters in data

| Parameter      |     Type |  Parameter Description   |
| :-------- | :--------| :------ |
| guid|  string| 节点ID|
| ip|  string| IP|
| country |  string| 区域|
| delay |  string| 延迟|

***

#### 启动VPN: node_start

> Requests and Parameter Description

| Parameter      |     Type |  Parameter Description   |
| :-------- | :--------| :------ |
| token | String | Token |
| guid |  string| 节点ID|

>Response and Parameters in data
>
>not data

***

#### 结束VPN: node_end

> 
>Requests and Parameter Description

| Parameter      |     Type |  Parameter Description   |
| :-------- | :--------| :------ |
| token |  string| 客户端公钥|
| guid | String | 节点id |

>Response and Parameters in data
>not data

***



#### 选择最快线路: ping

> Requests and Parameter Description

| Parameter | Type       | Must | Parameter Description |
| :-------- | :--------- | :--- | :-------------------- |
| ips       | String数组 | 是   | IP地址数组            |

>Response and Parameters in data

| Parameter | Type   | Parameter Description |
| :-------- | :----- | :-------------------- |
| ip        | String | IP                    |
| delay     | f64    | 延迟                  |

***



## Event

### Disconnected(String)

> 断开连接
>
> > data: 错误码与信息
> >
> > 格式：code：错误码。  reason：错误信息

#### 错误码对照表

| Code | Reason                        | Parameter Description |
| :--- | :---------------------------- | :-------------------- |
| 9000 | The connection is inactive    | 连接不活跃            |
| 9001 | Package has expired           | 套餐已过期            |
| 9002 | The connection is forced down | 该连接被强制下线      |



### UpdateToken(String)

> token更新
>
> > data: token

### PublicDBInitialized

> 公共sqlite db初始化成功
>
> > no data

### PublicDBUninitialized(String)

> 公共sqlite db初始化失败
>
> > data: 错误信息

