# Project Protocol

* 按照投资的`投融管退`的逻辑来组织`项目`的数据结构。

* `项目`样板数据如下：

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

* 主要由这些数据结构构成`项目阶段`，`待办事项`，`资产变化`，`AI审核`，`King审批`。

## 项目阶段

## 待办事项

## 资产变化

## AI审核

## King审批