var searchIndex = {};
searchIndex["plaid"] = {"doc":"[Plaid][Plaid] is the technology layer for financial services.\nThis library makes it easy to interface with Plaid.","items":[[0,"api","plaid","The namespace that everything in this library falls under.",null,null],[0,"user","plaid::api","User-related data structures.",null,null],[3,"User","plaid::api::user","# User\nRepresents an authorized user for a given product.",null,null],[12,"access_token","","The access token for this user",0,null],[11,"fmt","","",0,{"inputs":[{"name":"user"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"decode","","",0,{"inputs":[{"name":"d"}],"output":{"name":"result"}}],[0,"client","plaid::api","Data structures and methods that interact with Plaid via HTTP.",null,null],[3,"Client","plaid::api::client","# Client",null,null],[12,"endpoint","","E.g `https://api.plaid.com`.",1,null],[12,"client_id","","Your application&#39;s `client_id`.",1,null],[12,"secret","","Your application&#39;s `secret`.",1,null],[12,"hyper","","The instance of `hyper::Client` to use.\n*In most cases* you simply need `hyper::Client::new()`.\nHowever this is a good place to configure things like\nproxies, timeouts etc.",1,null],[0,"payload","","Payload",null,null],[3,"AuthenticateOptions","plaid::api::client::payload","Options that can be passed along to any `Payload::Authenticate` request.",null,null],[3,"FetchDataOptions","","Options that can be passed along to any `Payload::FetchData` request.",null,null],[4,"Payload","","Use this enum to tell the client what you want to do\nwith the associated product.",null,null],[13,"Authenticate","","Authenticate a user.",2,null],[13,"Reauthenticate","","Re-euthenticate an existing user.",2,null],[13,"Upgrade","","Upgrade the user for access to the given product.",2,null],[13,"RemoveUser","","Delete a user from Plaid.",2,null],[13,"StepMFA","","Send multifactor authentication response.",2,null],[13,"FetchData","","Retrieve data from the product.",2,null],[4,"SelectedDevice","","The device that the user has chosen to use for mfa.",null,null],[13,"Mask","","The `mask` returned when authenticating with `AuthenticateOptions { list: true, .. }`,\ne.g &quot;t..t@plaid.com&quot;,",3,null],[13,"Device","","The type of the device as defined under `mfa::Device`.",3,null],[11,"endpoint","","Returns the desired endpoint of the payload, given a `Product`",2,{"inputs":[{"name":"payload"},{"name":"client"},{"name":"p"}],"output":{"name":"string"}}],[11,"method","","Returns the `hyper::method::Method` to be used for the request",2,{"inputs":[{"name":"payload"}],"output":{"name":"method"}}],[11,"encode","","",2,{"inputs":[{"name":"payload"},{"name":"s"}],"output":{"name":"result"}}],[11,"fmt","","",3,{"inputs":[{"name":"selecteddevice"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"encode","","",3,{"inputs":[{"name":"selecteddevice"},{"name":"e"}],"output":{"name":"result"}}],[11,"encode","","",4,{"inputs":[{"name":"authenticateoptions"},{"name":"__s"}],"output":{"name":"result"}}],[11,"fmt","","",4,{"inputs":[{"name":"authenticateoptions"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"default","","Generate a default `AuthenticateOptions` struct with every field unset.",4,{"inputs":[],"output":{"name":"authenticateoptions"}}],[11,"encode","","",5,{"inputs":[{"name":"fetchdataoptions"},{"name":"__s"}],"output":{"name":"result"}}],[11,"fmt","","",5,{"inputs":[{"name":"fetchdataoptions"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"default","","Generate a default `FetchDataOptions` struct with every field unset.",5,{"inputs":[],"output":{"name":"fetchdataoptions"}}],[0,"response","plaid::api::client","Response",null,null],[4,"Response","plaid::api::client::response","# Response",null,null],[13,"MFA","","Waiting on multifactor authentication code from the user",6,null],[13,"ProductNotEnabled","","Returned when a request is made for a `Product` that is not\ncurrently enabled for the given `User`.",6,null],[13,"ProductData","","We have sucessfully fetched the available data pertaining to the\ngiven `Product`.",6,null],[13,"Authenticated","","We have successfully authenticated the user, and have retrieved\nthe relevant `Product::Data` along with that authentication.",6,null],[13,"Unknown","","Nothing is known about the user and no requests have been made",6,null],[11,"fmt","","",6,{"inputs":[{"name":"response"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"clone","plaid::api::client","",1,{"inputs":[{"name":"client"}],"output":{"name":"client"}}],[11,"request","","Make a request to the given [Product](../product/struct.Product.html), using a\n[Payload](./payload/struct.Payload.html) describing the intention of the operation.",1,{"inputs":[{"name":"client"},{"name":"p"},{"name":"payload"}],"output":{"name":"result"}}],[0,"error","plaid::api","Namespace for error definitions. Responses are only considered to\nbe errors if they fall outside of the expected user flow. By that\ndefinition, all non 2XX HTTP response codes are considered an error.",null,null],[4,"Error","plaid::api::error","# Error\nRepresents possible errors returned from the API.\n`P` represents the product that the error is scoped for.",null,null],[13,"UnsuccessfulResponse","","Represents bad HTTP status codes, or codes that we don&#39;t support.",7,null],[13,"InvalidResponse","","Represents errors forwarded from `rustc_serialize`, usually indicating\nthat the response returned something that could not be decoded.",7,null],[13,"HTTP","","Represents an error forwarded from `hyper`, which means it is most\nlikely HTTP (protocol, rather than status code) related.",7,null],[13,"IO","","Returned for errors that are forwarded from `std::io::Error`",7,null],[13,"InternalError","","This should happen very rarely, and indicates that something is most\nlikely wrong with `plaid::api` rather than the end user.",7,null],[11,"fmt","","",7,{"inputs":[{"name":"error"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"fmt","","",7,{"inputs":[{"name":"error"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"description","","",7,{"inputs":[{"name":"error"}],"output":{"name":"str"}}],[11,"from","","",7,{"inputs":[{"name":"ioerror"}],"output":{"name":"error"}}],[11,"from","","",7,{"inputs":[{"name":"error"}],"output":{"name":"error"}}],[11,"from","","",7,{"inputs":[{"name":"decodererror"}],"output":{"name":"error"}}],[11,"from","","",7,{"inputs":[{"name":"encodererror"}],"output":{"name":"error"}}],[0,"product","plaid::api","Product definitions that provide endpoint and response deserialization\ninformation.",null,null],[0,"connect","plaid::api::product","Connect is a product that Plaid offers. It allows you to retrieve account balance\nand transaction history data.",null,null],[3,"Connect","plaid::api::product::connect","`Connect` is the endpoint you need to fetch transaction for a `User`",null,null],[3,"ConnectData","","Representation of data that is retrieved from the `Connect` product.",null,null],[12,"accounts","","List of accounts associated with the user",8,null],[12,"transactions","","List of transactions associated with the user",8,null],[11,"fmt","","",9,{"inputs":[{"name":"connect"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"decode","","",8,{"inputs":[{"name":"__d"}],"output":{"name":"result"}}],[11,"fmt","","",8,{"inputs":[{"name":"connectdata"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"description","","",9,{"inputs":[{"name":"connect"}],"output":{"name":"str"}}],[11,"endpoint","","",9,{"inputs":[{"name":"connect"},{"name":"payload"}],"output":{"name":"str"}}],[0,"auth","plaid::api::product","Auth is a product that allows you to authorize ACH transaction from the\nend-users account. It will return account data including account numbers\nand routing numbers if authorization is successful.",null,null],[3,"Auth","plaid::api::product::auth","`Auth` is the endpoint you need in order to check that the user owns their account.",null,null],[3,"AuthData","","Representation of data that is retrieved from the `Auth` product.",null,null],[12,"accounts","","List of accounts associated with the user",10,null],[11,"fmt","","",11,{"inputs":[{"name":"auth"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"decode","","",10,{"inputs":[{"name":"__d"}],"output":{"name":"result"}}],[11,"fmt","","",10,{"inputs":[{"name":"authdata"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"description","","",11,{"inputs":[{"name":"auth"}],"output":{"name":"str"}}],[11,"endpoint","","",11,{"inputs":[{"name":"auth"},{"name":"payload"}],"output":{"name":"str"}}],[0,"info","plaid::api::product","The Info endpoint allows you to retrieve various account holder\ninformation on file with the financial institution, including names, emails,\nphone numbers, and addresses.",null,null],[3,"Info","plaid::api::product::info","The definition of the Info Product.",null,null],[3,"InfoData","","Representation of data that is retrieved from the `Info` product.",null,null],[12,"accounts","","List of accounts associated with the user. When returned from the\nInfo endpoint it will also include account and routing numbers.",12,null],[12,"info","","Includes all user information that has been returned.",12,null],[3,"InfoInternalData","","Represents the *actual* info data from an info response.",null,null],[12,"emails","","Emails associated with the user.",13,null],[12,"addresses","","Addresses associated with the user.",13,null],[12,"phone_numbers","","Phone numbers associated with the user.",13,null],[3,"Email","","A user&#39;s email, including meta data returned by Plaid.",null,null],[12,"primary","","Whether or not the user has chosen this as their primary email.",14,null],[12,"email_type","","The designated type for this email (e.g personal, home).",14,null],[12,"email","","The actual email address.",14,null],[3,"Address","","A user&#39;s address, as returned by Plaid.",null,null],[12,"primary","","Whether or not the user has chosen this as their primary address",15,null],[12,"zip","","The address zip code part.",15,null],[12,"state","","The address state part.",15,null],[12,"city","","The address city part.",15,null],[12,"street","","The address street part.",15,null],[3,"PhoneNumber","","A user&#39;s phone number, as returned by Plaid.",null,null],[12,"primary","","Whether or not the user has chosen this as their primary phone number.",16,null],[12,"phone_number_type","","The type of the phone number (e.g personal, home).",16,null],[12,"phone_number","","The actual phone number.",16,null],[11,"fmt","","",17,{"inputs":[{"name":"info"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"decode","","",12,{"inputs":[{"name":"__d"}],"output":{"name":"result"}}],[11,"fmt","","",12,{"inputs":[{"name":"infodata"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"decode","","",13,{"inputs":[{"name":"__d"}],"output":{"name":"result"}}],[11,"fmt","","",13,{"inputs":[{"name":"infointernaldata"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"description","","",17,{"inputs":[{"name":"info"}],"output":{"name":"str"}}],[11,"endpoint","","",17,{"inputs":[{"name":"info"},{"name":"payload"}],"output":{"name":"str"}}],[11,"fmt","","",14,{"inputs":[{"name":"email"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"decode","","",14,{"inputs":[{"name":"d"}],"output":{"name":"result"}}],[11,"fmt","","",15,{"inputs":[{"name":"address"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"decode","","",15,{"inputs":[{"name":"d"}],"output":{"name":"result"}}],[11,"fmt","","",16,{"inputs":[{"name":"phonenumber"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"decode","","",16,{"inputs":[{"name":"d"}],"output":{"name":"result"}}],[0,"balance","plaid::api::product","Balance is a product that allows users to query the account balance\nof a given user.",null,null],[3,"Balance","plaid::api::product::balance","`Balance` is the endpoint you need to fetch transaction for a `User`",null,null],[3,"BalanceData","","Representation of data that is retrieved from the `Balance` product.",null,null],[12,"accounts","","List of accounts associated with the user",18,null],[11,"fmt","","",19,{"inputs":[{"name":"balance"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"decode","","",18,{"inputs":[{"name":"__d"}],"output":{"name":"result"}}],[11,"fmt","","",18,{"inputs":[{"name":"balancedata"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"description","","",19,{"inputs":[{"name":"balance"}],"output":{"name":"str"}}],[11,"endpoint","","",19,{"inputs":[{"name":"balance"},{"name":"payload"}],"output":{"name":"str"}}],[0,"income","plaid::api::product","The Income endpoint allows you to retrieve various information pertaining to a user&#39;s income.",null,null],[3,"Income","plaid::api::product::income","The Income endpoint.",null,null],[3,"IncomeData","","The underlying data representation of Income.",null,null],[12,"accounts","","A list of user accounts and their balances.",20,null],[12,"income","","Income data",20,null],[3,"IncomeInternalData","","Internal data representation of the income response",null,null],[12,"income_streams","","A list of income streams.",21,null],[12,"last_year_income","","The sum of user&#39;s income over the past 365 days.\nIf we have less than 365 days of data this will be less than a full year&#39;s income.",21,null],[12,"last_year_income_before_tax","","`last_year_income` interpolated to value before taxes.\nThis is the minimum pre-tax salary that assumes a filing status of\nsingle with zero dependents.",21,null],[12,"projected_yearly_income","","User&#39;s income extrapolated over a year based on current,\nactive income streams. Income streams become inactive if they have not\nrecurred for more than two cycles. For example, if a weekly paycheck hasn&#39;t\nbeen seen for the past two weeks, it is no longer active.",21,null],[12,"projected_yearly_income_before_tax","","`projected_yearly_income` interpolated to value before taxes.\nThis is the minimum pre-tax salary that assumes a filing status of\nsingle with zero dependents.",21,null],[12,"max_number_of_overlapping_income_streams","","Max number of income streams present at the same time over the past 365 days.",21,null],[12,"number_of_income_streams","","Total number of distinct income streams received over the past 365 days.",21,null],[3,"IncomeStream","","An income stream represents a stream of income that Plaid\nhas detected from their transactions.",null,null],[12,"monthly_income","","How much income per month in dollars.",22,null],[12,"confidence","","Plaid&#39;s confidence in this estimate.",22,null],[12,"days","","The number of days Plaid has seen this for.",22,null],[12,"name","","The name of the income stream.",22,null],[11,"fmt","","",23,{"inputs":[{"name":"income"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"decode","","",20,{"inputs":[{"name":"__d"}],"output":{"name":"result"}}],[11,"fmt","","",20,{"inputs":[{"name":"incomedata"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"description","","",23,{"inputs":[{"name":"income"}],"output":{"name":"str"}}],[11,"endpoint","","",23,{"inputs":[{"name":"income"},{"name":"payload"}],"output":{"name":"str"}}],[11,"decode","","",21,{"inputs":[{"name":"__d"}],"output":{"name":"result"}}],[11,"fmt","","",21,{"inputs":[{"name":"incomeinternaldata"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"decode","","",22,{"inputs":[{"name":"__d"}],"output":{"name":"result"}}],[11,"fmt","","",22,{"inputs":[{"name":"incomestream"},{"name":"formatter"}],"output":{"name":"result"}}],[8,"Product","plaid::api::product","Anything that implements `Product` can be used as a product.",null,null],[16,"Data","","The response data that is associated with this product",24,null],[10,"endpoint","","The endpoint of the product for the given payload.\nWith leading slash, e.g `/connect/get`",24,{"inputs":[{"name":"product"},{"name":"payload"}],"output":{"name":"str"}}],[10,"description","","A textual representation of the product, e.g `Connect`",24,{"inputs":[{"name":"product"}],"output":{"name":"str"}}],[0,"mfa","plaid::api","Data structures and implementations related to multi-factor-authentication.",null,null],[4,"Challenge","plaid::api::mfa","Represents one of the different types of multi-factor-authentication\nchallenges Plaid supports.",null,null],[13,"Code","","A token-based authorization, this token will be sent to one of\nthe user&#39;s registered devices.",25,null],[13,"DeviceList","","A list of possible challenge devices, in which the user should\nchoose one and then pass along the selection using `api::client::payload::AuthenticateOptions`.\nIt is in the form of `(device_type, device_mask)`.",25,null],[13,"Questions","","A list of questions that need to be answered.",25,null],[13,"Selections","","A list of multi-choice selections",25,null],[4,"Response","","Represents a response to a previously given MFA challenge.",null,null],[13,"Code","","A response to a code challenge, providing the code\nthat was sent to the user&#39;s device.",26,null],[13,"Questions","","Responses to a previously given list of questions.",26,null],[13,"Selections","","Responses to a previously given list of selections.",26,null],[4,"Device","","Represents a device that can be used for multifactor authentication",null,null],[13,"Email","","Code sent to the user&#39;s email",27,null],[13,"Phone","","Code sent to the user&#39;s phone number via sms",27,null],[13,"Card","","Verify a credit card number",27,null],[11,"eq","","",25,{"inputs":[{"name":"challenge"},{"name":"challenge"}],"output":{"name":"bool"}}],[11,"ne","","",25,{"inputs":[{"name":"challenge"},{"name":"challenge"}],"output":{"name":"bool"}}],[11,"fmt","","",25,{"inputs":[{"name":"challenge"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"eq","","",26,{"inputs":[{"name":"response"},{"name":"response"}],"output":{"name":"bool"}}],[11,"ne","","",26,{"inputs":[{"name":"response"},{"name":"response"}],"output":{"name":"bool"}}],[11,"fmt","","",26,{"inputs":[{"name":"response"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"encode","","",26,{"inputs":[{"name":"response"},{"name":"e"}],"output":{"name":"result"}}],[11,"decode","","",25,{"inputs":[{"name":"d"}],"output":{"name":"result"}}],[11,"clone","","",27,{"inputs":[{"name":"device"}],"output":{"name":"device"}}],[11,"eq","","",27,{"inputs":[{"name":"device"},{"name":"device"}],"output":{"name":"bool"}}],[11,"fmt","","",27,{"inputs":[{"name":"device"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"decode","","",27,{"inputs":[{"name":"d"}],"output":{"name":"result"}}],[11,"encode","","",27,{"inputs":[{"name":"device"},{"name":"e"}],"output":{"name":"result"}}],[0,"data","plaid::api","Types that define data structures that are returned from the API.",null,null],[0,"account","plaid::api::data","Representations of a user&#39;s bank account.",null,null],[3,"Account","plaid::api::data::account","# Account\nRepresents one account associated with the given `User`.",null,null],[12,"id","","The unique id of the account.",28,null],[12,"item_id","","An id unique to the accounts of a particular access token",28,null],[12,"current_balance","","The total amount of funds in the account",28,null],[12,"available_balance","","The Current Balance less any outstanding holds or debits that\nhave not yet posted to the account. May sometimes not be available.",28,null],[12,"institution","","The financial institution associated with the account.",28,null],[12,"account_type","","The classification of this account.\n[See here for more info](https://plaid.com/docs/api/#connect-account-types).",28,null],[12,"account_subtype","","A more detailed classification of the account.\nThis is not always available.\n[See here for a list of possible types][sub-types].\n[sub-types]: https://plaid.com/docs/api/#connect-account-subtypes",28,null],[12,"account_number","","The user&#39;s bank account number.\nOnly available when using `api::product::Auth`.",28,null],[12,"routing_number","","The user&#39;s routing number.\nOnly available when using `api::product::Auth`.",28,null],[12,"wire_routing_number","","The user&#39;s wire routing number.\nOnly available when using `api::product::Auth`.",28,null],[11,"fmt","","",28,{"inputs":[{"name":"account"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"decode","","",28,{"inputs":[{"name":"d"}],"output":{"name":"result"}}],[0,"transaction","plaid::api::data","Representations of banking transactions.",null,null],[3,"Transaction","plaid::api::data::transaction","# Transaction\nRepresents a single transaction associated with a given `Account`.",null,null],[12,"id","","The unique identifier of this transaction.",29,null],[12,"account_id","","The associated `Account`.",29,null],[12,"amount","","Dollar value as as float. It is positive to indicate money\nmoving out of the account, and negative to indicate that\nmoney is moving in.",29,null],[12,"category_id","","The category to which this account belongs.\n[A list can be found here](https://plaid.com/docs/api/#all-categories).",29,null],[12,"context","","The context in which the transaction occurred.",29,null],[12,"categories","","An hierarchical list of the categories in which\nthis transaction belongs to.",29,null],[12,"pending","","When `true`, then this transaction is cleared and immutable.\nWhen `false`, then it is posted and subject to change in the future.",29,null],[12,"date","","The date on which the transaction took place.\nPlaid standardizes using the ISO 8601 format.",29,null],[4,"Context","","The context in which a transaction took place",null,null],[13,"Place","","A phyical place",30,null],[13,"Digital","","An online transaction",30,null],[13,"Special","","Usually banking transactions",30,null],[13,"Unresolved","","Could not be determined",30,null],[11,"fmt","","",29,{"inputs":[{"name":"transaction"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"decode","","",29,{"inputs":[{"name":"d"}],"output":{"name":"result"}}],[11,"fmt","","",30,{"inputs":[{"name":"context"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"decode","","",30,{"inputs":[{"name":"d"}],"output":{"name":"result"}}],[0,"types","plaid::api::data","Type aliases that map to common types.",null,null],[6,"UID","plaid::api::data::types","Unique identifiers in Plaid are represented as a globally unique hash.",null,null],[6,"CategoryID","","Category identifiers are represented by an unsigned integer.",null,null],[6,"Amount","","All amounts are represented in a 64-bit floating-point type.\nThis is for legacy reasons and may change in the future.",null,null],[6,"Username","","A user&#39;s bank account username.",null,null],[6,"Name","","A user&#39;s real namee.",null,null],[6,"Password","","A user&#39;s bank account password.",null,null],[6,"MFACode","","A user&#39;s multi-factor authentication code.",null,null],[6,"AccessToken","","A user&#39;s secret access token",null,null],[6,"ClientID","","Your client id from the Plaid dashboard.",null,null],[6,"ClientSecret","","Your client secret from the Plaid dashboard.",null,null],[6,"Institution","","A user&#39;s institution. See [here for a list](https://plaid.com/docs/api/#institutions).",null,null],[6,"PIN","","A PIN number",null,null],[6,"Date","","Dates are simply stored as their original `String` representation.\nIt is up to you to parse it with your favorite date/time library.",null,null]],"paths":[[3,"User"],[3,"Client"],[4,"Payload"],[4,"SelectedDevice"],[3,"AuthenticateOptions"],[3,"FetchDataOptions"],[4,"Response"],[4,"Error"],[3,"ConnectData"],[3,"Connect"],[3,"AuthData"],[3,"Auth"],[3,"InfoData"],[3,"InfoInternalData"],[3,"Email"],[3,"Address"],[3,"PhoneNumber"],[3,"Info"],[3,"BalanceData"],[3,"Balance"],[3,"IncomeData"],[3,"IncomeInternalData"],[3,"IncomeStream"],[3,"Income"],[8,"Product"],[4,"Challenge"],[4,"Response"],[4,"Device"],[3,"Account"],[3,"Transaction"],[4,"Context"]]};
initSearch(searchIndex);
