pub const NAME: &str = "Binance";
pub const API_KEY: &str = "mdKsfufQnWoMtf41XI8HNfOfz7fXLf8Xa3WoSbGeZgfEFaMlUw4HOtu6wzRxwAto";
pub const SECRET_KEY: &str = "mLSl2OxVYANgRolrMzWYdx09DTVwWWRbgPJogXuVQMtSLK3VnyFbIyBhl8WuB8vz";
pub const BASE_URL: &str = "https://api-gcp.binance.com";
pub const WEBSOCKET_URL: &str = "wss://stream.binance.com:9443/ws";

// const FETCH_DEPOSIT_ADDRESS_WITH_NETWORKS: &str = "/sapi/v1/capital/deposit/address/list";
// // REQUEST
// // Name	                Type	            Mandatory	            Description
// // coin	                STRING	            YES	                    coin refers to the parent network address format that the address is using
// // network	            STRING	            NO
// // timestamp	        LONG	            YES
// // RESPONSE
// // [
// //   {
// //     "coin": "ETH", //coin here means network address space, ETH for all EVM-like network
// //     "address": "0xD316E95Fd9E8E237Cb11f8200Babbc5D8D177BA4",
// //     "tag":"",
// //     "isDefault": 0
// //   },
// //   {
// //     "coin": "ETH",
// //     "address": "0xD316E95Fd9E8E237Cb11f8200Babbc5D8D177BA4",
// //     "tag":"",
// //     "isDefault": 0
// //   },
// //   {
// //     "coin": "ETH",
// //     "address": "0x00003ada75e7da97ba0db2fcde72131f712455e2",
// //     "tag":"",
// //     "isDefault": 1  //'isDefault' is 1 means the address is default, same as shown in the app.
// //   }
// // ]
// pub const FETCH_NETWORKS: &str = "/sapi/v1/capital/config/getall";
// // REQUEST
// // Name                 Type                Mandatory               Description
// // recvWindow	        LONG	            NO
// // timestamp	        LONG	            YES
// // RESPONSE
// // [
// //     {
// //         "coin": "1MBABYDOGE",
// //         "depositAllEnable": true,
// //         "withdrawAllEnable": true,
// //         "name": "1M x BABYDOGE",
// //         "free": "34941.1",
// //         "locked": "0",
// //         "freeze": "0",
// //         "withdrawing": "0",
// //         "ipoing": "0",
// //         "ipoable": "0",
// //         "storage": "0",
// //         "isLegalMoney": false,
// //         "trading": true,
// //         "networkList": [
// //             {
// //                 "network": "BSC",
// //                 "coin": "1MBABYDOGE",
// //                 "withdrawIntegerMultiple": "0.01",
// //                 "isDefault": false,
// //                 "depositEnable": true,
// //                 "withdrawEnable": true,
// //                 "depositDesc": "",   // shown only when "depositEnable" is false.
// //                 "withdrawDesc": "",  // shown only when "withdrawEnable" is false.
// //                 "specialTips": "",
// //                 "specialWithdrawTips": "",
// //                 "name": "BNB Smart Chain (BEP20)",
// //                 "resetAddressStatus": false,
// //                 "addressRegex": "^(0x)[0-9A-Fa-f]{40}$",
// //                 "memoRegex": "",
// //                 "withdrawFee": "10",
// //                 "withdrawMin": "20",
// //                 "withdrawMax": "9999999999",
// //                 "withdrawInternalMin": "0.01", // Minimum internal transfer amount
// //                 "depositDust": "0.01",
// //                 "minConfirm": 5,  // min number for balance confirmation
// //                 "unLockConfirm": 0,  // confirmation number for balance unlock
// //                 "sameAddress": false,  // Obsoleted, recomment to use withdrawTag
// //                 "withdrawTag": false, // If the coin needs to provide memo to withdraw
// //                 "estimatedArrivalTime": 1,
// //                 "busy": false,
// //                 "contractAddressUrl": "https://bscscan.com/token/",
// //                 "contractAddress": "0xc748673057861a797275cd8a068abb95a902e8de",
// //                 "denomination": 1000000   // 1 1MBABYDOGE = 1000000 BABYDOGE
// //             },
// //             {
// //                 "network": "ETH",
// //                 "coin": "1MBABYDOGE",
// //                 "withdrawIntegerMultiple": "0.01",
// //                 "isDefault": true,
// //                 "depositEnable": true,
// //                 "withdrawEnable": true,
// //                 "depositDesc": "",
// //                 "withdrawDesc": "",
// //                 "specialTips": "",
// //                 "specialWithdrawTips": "",
// //                 "name": "Ethereum (ERC20)",
// //                 "resetAddressStatus": false,
// //                 "addressRegex": "^(0x)[0-9A-Fa-f]{40}$",
// //                 "memoRegex": "",
// //                 "withdrawFee": "1511",
// //                 "withdrawMin": "3022",
// //                 "withdrawMax": "9999999999",
// //                 "withdrawInternalMin": "0.01",
// //                 "depositDust": "0.01",
// //                 "minConfirm": 6,
// //                 "unLockConfirm": 64,
// //                 "sameAddress": false,
// //                 "withdrawTag": false,
// //                 "estimatedArrivalTime": 2,
// //                 "busy": false,
// //                 "contractAddressUrl": "https://etherscan.io/address/",
// //                 "contractAddress": "0xac57de9c1a09fec648e93eb98875b212db0d460b",
// //                 "denomination": 1000000
// //             }
// //         ]
// //     }
// // ]
// const FETCH_WALLET_ASSETS: &str = "/sapi/v3/asset/getUserAsset";
// // REQUEST
// // Name	                Type	            Mandatory	            Description
// // asset	            STRING	            NO	                    If asset is blank, then query all positive assets user have.
// // needBtcValuation	    BOOLEAN         	NO	                    Whether need btc valuation or not.
// // recvWindow	        LONG	            NO
// // timestamp	        LONG	            YES
// // RESPONSE
// // [
// //   {
// //     "asset": "AVAX",
// //     "free": "1",
// //     "locked": "0",
// //     "freeze": "0",
// //     "withdrawing": "0",
// //     "ipoable": "0",
// //     "btcValuation": "0"
// //   },
// //   {
// //     "asset": "BCH",
// //     "free": "0.9",
// //     "locked": "0",
// //     "freeze": "0",
// //     "withdrawing": "0",
// //     "ipoable": "0",
// //     "btcValuation": "0"
// //   },
// //   {
// //     "asset": "BNB",
// //     "free": "887.47061626",
// //     "locked": "0",
// //     "freeze": "10.52",
// //     "withdrawing": "0.1",
// //     "ipoable": "0",
// //     "btcValuation": "0"
// //   },
// //   {
// //     "asset": "BUSD",
// //     "free": "9999.7",
// //     "locked": "0",
// //     "freeze": "0",
// //     "withdrawing": "0",
// //     "ipoable": "0",
// //     "btcValuation": "0"
// //   },
// //   {
// //     "asset": "SHIB",
// //     "free": "532.32",
// //     "locked": "0",
// //     "freeze": "0",
// //     "withdrawing": "0",
// //     "ipoable": "0",
// //     "btcValuation": "0"
// //   },
// //   {
// //     "asset": "USDT",
// //     "free": "50300000001.44911105",
// //     "locked": "0",
// //     "freeze": "0",
// //     "withdrawing": "0",
// //     "ipoable": "0",
// //     "btcValuation": "0"
// //   },
// //   {
// //     "asset": "WRZ",
// //     "free": "1",
// //     "locked": "0",
// //     "freeze": "0",
// //     "withdrawing": "0",
// //     "ipoable": "0",
// //     "btcValuation": "0"
// //   }
// // ]
// const MAKE_WITHDRAW: &str = "/sapi/v1/capital/withdraw/apply";
// // REQUEST
// // Name	                Type	            Mandatory	            Description
// // coin             	STRING	            YES
// // withdrawOrderId	    STRING	            NO	                    client side id for withdrawal, if provide here, can be used in GET /sapi/v1/capital/withdraw/history for query.
// // network	            STRING	            NO
// // address	            STRING	            YES
// // addressTag	        STRING	            NO	                    Secondary address identifier for coins like XRP,XMR etc.
// // amount	            DECIMAL	            YES
// // transactionFeeFlag	BOOLEAN	            NO	                    When making internal transfer, true for returning the fee to the destination account; false for returning the fee back to the departure account. Default false.
// // name	                STRING	            NO	                    Description of the address. Address book cap is 200, space in name should be encoded into %20
// // walletType	        INTEGER	            NO	                    The wallet type for withdraw，0-spot wallet ，1-funding wallet. Default walletType is the current "selected wallet" under wallet->Fiat and Spot/Funding->Deposit
// // recvWindow	        LONG	            NO
// // timestamp	        LONG	            YES
// // RESPONSE
// // {
// //     "id":"7213fea8e94b4a5593d507237e5a555b"
// // }

// // const FETCH_ASSET_DETAIL: &str = "/sapi/v1/asset/assetDetail";
// // // REQUEST
// // // Name	                Type	            Mandatory	            Description
// // // recvWindow	        LONG            	NO
// // // timestamp	        LONG            	YES
// // // RESPONSE
// // // {
// // //         "CTR": {
// // //             "minWithdrawAmount": "70.00000000", //min withdraw amount
// // //             "depositStatus": false,//deposit status (false if ALL of networks' are false)
// // //             "withdrawFee": 35, // withdraw fee
// // //             "withdrawStatus": true, //withdraw status (false if ALL of networks' are false)
// // //             "depositTip": "Delisted, Deposit Suspended" //reason
// // //         },
// // //         "SKY": {
// // //             "minWithdrawAmount": "0.02000000",
// // //             "depositStatus": true,
// // //             "withdrawFee": 0.01,
// // //             "withdrawStatus": true
// // //         }
// // // }

// const MAKE_BORROW_ASSET: &str = "/sapi/v1/margin/borrow-repay"; //POST
// const MAKE_REPAY_ASSET: &str = "/sapi/v1/margin/borrow-repay"; //POST
// // REQUEST
// // Name	                Type            	Mandatory           	Description
// // asset	            STRING          	YES
// // isIsolated	        STRING          	YES                 	TRUE for Isolated Margin, FALSE for Cross Margin, Default FALSE
// // symbol	            STRING          	YES                    	Only for Isolated margin
// // amount	            STRING          	YES
// // type	                STRING	            YES                 	BORROW or REPAY
// // recvWindow	        LONG            	NO                  	The value cannot be greater than 60000
// // timestamp	        LONG            	YES
// // RESPONSE
// //  {
// //   //transaction id
// //   "tranId": 100000001
// // }
// pub const FETCH_MARGIN_INFO: &str = "/sapi/v1/margin/allAssets";
// // REQUEST
// // Name	                Type            	Mandatory           	Description
// // asset	            STRING          	NO
// // RESPONSE
// // [
// //   {
// //     "assetFullName": "USD coin",
// //     "assetName": "USDC",
// //     "isBorrowable": true,
// //     "isMortgageable": true,
// //     "userMinBorrow": "0.00000000",
// //     "userMinRepay": "0.00000000",
// //     "delistTime": 1704973040
// //   }
// // ]

// const MAKE_TRANSFER_ASSET: &str = "/sapi/v1/asset/transfer"; //POST
// // REQUEST
// //  Name	                Type	            Mandatory           	Description
// // type	                ENUM	            YES                   	[MAIN_MARGIN, MARGIN_MAIN, FUNDING_MAIN, MAIN_FUNDING, FUNDGING_MARGIN, MARGIN_FUNDING]
// // asset	            STRING          	YES
// // amount	            DECIMAL         	YES
// // fromSymbol	        STRING          	NO
// // toSymbol	            STRING          	NO
// // recvWindow	        LONG            	NO
// // timestamp	        LONG            	YES
// // RESPONSE
// //  {
// //     "tranId":13526853623
// // }
