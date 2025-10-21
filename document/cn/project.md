# Project Protocol

* 转移到文件夹`septopu/homepage/record/docs/zh/entry/`

* 按照投资的`投融管退`的逻辑来组织`项目`的数据结构。

* 任何人都可以创建项目，然后就开始走`milestone`,`项目`样板数据如下。

```Javascript
    {
        name:"PROJECT_NAME",
        desc:"DESCRIPTION_OF_PROJECT",
        founder:"SOLANA_ADDRESS",
        recipient:"SOLANA_ADDRESS",
        status:1,                           //[0.start; 1.normal;]
        milestone:[
            {
                todo:{
                    task:[      //待办事项包括到所有流程，例如，后期的财务报表上传。可以通过这些来辅助判断项目的情况。
                        {
                            category:1,                 //任务的分类
                            detail:"IPFS_ADDRESS",      //详情说明，需要放到IPFS里
                            start:"REAL_WORLD_TIME",
                            end:"REAL_WORLD_TIME",
                        },
                    ],
                    asset:[                             //当存在asset设置时，approve通过需要进行资产处理
                        {
                            from:"SOLANA_ADDRESSS",
                            to:"SOLANA_ADDRESSS",
                            token:"TOKEN_ADDRESS",
                            purpose:2,                  //用途的分类
                        },
                        ...
                    ]
                },
                start:1000,     //链上的开始block的高度
                end:0,          //链上该milestone结束时block的高度
                log:[           //审核过程的记录，存在多次审核的情况。
                    {
                        report:{
                            ipfs:"IPFS_ADDRESS_OF_MILESTONE_REPORT",
                            stamp:10002,
                            from:"",
                        },
                        judgement:{
                            AI:[{},{},{},{},{},{},{}],
                            result:true,
                        },
                        approve:{
                            agent:true,
                            address:"SOLANA_ADDRESSS",
                            result:true
                        },
                    },
                    ...
                ],
            },
        ]
    }
```

* 主要由这些数据结构构成`项目阶段`，`待办事项`，`资产变化`，`AI审核`，`King审批`。

## 数据结构说明

### 项目阶段

### 待办事项

### 资产变化

### AI审核

### King审批

## Complain

* 存在Project恶意行为，可以通过这部分来进行处理。可以由任何人发起，来实现监督。存在该功能被恶意使用，也需要进行谨慎细致的设计。

----

* 之前的设计，先放着

```Javascript
    {
        name:"PROJECT_NAME",
        desc:"",
        founder:"SOLANA_ADDRESS",
        asset:{
            pie:[
                {
                    holder:"",
                    desc:"",
                    amount:10000,
                    lock:0,
                    memo:{},
                },
                {
                    holder:"",
                    desc:"",
                    amount:20000,
                    lock:0,
                    memo:{},
                },
            ],
            address:"SOLANA_SPL_TOKEN_ADDRESS",
            status:1,
        },
        judgement:{
            proposol:"IPFS_ADDRESS_OF_PROJECT",
            ai:[],
            approve:"APPROVE_TRANSACTION_HASH",
        },
        status:1,               //project status
        milestone:[
            {
                name:"WHAT_TO_DO",
                todo:[

                ],
                asset:{                 //asset action after finish the job
                    token:"TOKEN_TYPE",             //token的名称
                    amount:0,                       //获取的数量
                    reciepent:"SOLANA_ADDRESSS",    //token接受者
                    lock:0,                         //token的锁定状态
                },
                judgement:{
                    proposol:"IPFS_ADDRESS_OF_PROJECT",
                    ai:[],
                    approve:{
                        agent:true,                  //是否由代理而不是king签署的
                        address:"SOLANA_ADDRESSS",
                        transaction:"TRANSACTION_HASH_OF_SIGN",
                    }
                },
            },
        ],
    }
```
