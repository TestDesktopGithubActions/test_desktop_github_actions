type LoginInput = { email: String, passwd: String };
type LoginOutput = "String";
type LoginCommand = { name: 'login', input: LoginInput, output: LoginOutput };

type LoginTemporaryInput = { email: String, passwd: String, proof: String };
type LoginTemporaryOutput = "String";
type LoginTemporaryCommand = { name: 'login_temporary', input: LoginTemporaryInput, output: LoginTemporaryOutput };

type LogoutInput = { token: String };
type LogoutOutput = "String";
type LogoutCommand = { name: 'logout', input: LogoutInput, output: LogoutOutput };

type RegisterInput = { email: String, passwd: String, repeatPassword: String };
type RegisterOutput = "String";
type RegisterCommand = { name: 'register', input: RegisterInput, output: RegisterOutput };

type BindDeviceInput = {};
type BindDeviceOutput = "String";
type BindDeviceCommand = { name: 'bind_device', input: BindDeviceInput, output: BindDeviceOutput };

type ActivatingInput = { accountCode: String, code: String };
type ActivatingOutput = "String";
type ActivatingCommand = { name: 'activating', input: ActivatingInput, output: ActivatingOutput };

type AccountUpdateTokenInput = { token: String };
type AccountUpdateTokenOutput = "String";
type AccountUpdateTokenCommand = { name: 'account_update_token', input: AccountUpdateTokenInput, output: AccountUpdateTokenOutput };

type NodeListInput = { token: String };
type NodeListOutput = "String";
type NodeListCommand = { name: 'node_list', input: NodeListInput, output: NodeListOutput };

type NodeStartInput = { token: String, guid: String };
type NodeStartOutput = "String";
type NodeStartCommand = { name: 'node_start', input: NodeStartInput, output: NodeStartOutput };

type NodeEndInput = { token: String, guid: String };
type NodeEndOutput = "String";
type NodeEndCommand = { name: 'node_end', input: NodeEndInput, output: NodeEndOutput };

type UploadLogInput = { token: String, email: String };
type UploadLogOutput = "String";
type UploadLogCommand = { name: 'upload_log', input: UploadLogInput, output: UploadLogOutput };

type PingInput = { ips: Vec<String> };
type PingOutput = "String";
type PingCommand = { name: 'ping', input: PingInput, output: PingOutput };

type GetInfoInput = {};
type GetInfoOutput = "String";
type GetInfoCommand = { name: 'get_info', input: GetInfoInput, output: GetInfoOutput };

type SetLanguageInput = { language: String };
type SetLanguageOutput = "String";
type SetLanguageCommand = { name: 'set_language', input: SetLanguageInput, output: SetLanguageOutput };

type GetPkInput = {};
type GetPkOutput = "String";
type GetPkCommand = { name: 'get_pk', input: GetPkInput, output: GetPkOutput };

type SplashscreenInput = {};
type SplashscreenOutput = "String";
type SplashscreenCommand = { name: 'splashscreen', input: SplashscreenInput, output: SplashscreenOutput };

type Command = LoginCommand | LoginTemporaryCommand | LogoutCommand | RegisterCommand | BindDeviceCommand | ActivatingCommand | AccountUpdateTokenCommand | NodeListCommand | NodeStartCommand | NodeEndCommand | UploadLogCommand | PingCommand | GetInfoCommand | SetLanguageCommand | GetPkCommand | SplashscreenCommand;