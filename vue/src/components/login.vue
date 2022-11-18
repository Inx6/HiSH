<template>
    <div>
        <mu-container>
            <mu-row gutter style="background-color:rgba(225,225,224,0.3)">
                <mu-col :span="12" style="margin-top:10em;margin-bottom:8em">
                     <mu-flex justify-content="center">
                         <h1>Welcome to&nbsp;</h1><h1 style="color:pink">Shinx!</h1>
                    </mu-flex>
                    <mu-flex justify-content="center">
                        <h2>{{title}}</h2>
                    </mu-flex>
                </mu-col>
                <mu-col :span="3"></mu-col>
                <mu-col :span="6">
                    <div  class="login_main">
                        <mu-flex justify-content="center">
                            <mu-text-field v-model="login.username" label="姓名" suffix="name"></mu-text-field>
                        </mu-flex>
                        <mu-flex justify-content="center">
                            <mu-text-field v-model="login.password" label="密码" suffix="" :action-icon="visibility ? '隐藏' : '显示'" :action-click="() => (visibility = !visibility)" :type="visibility ? 'text' : 'password'"></mu-text-field>
                        </mu-flex>
                    </div>
                    <div class="login_other" v-if="show">
                        <mu-flex justify-content="center">
                            <mu-text-field v-model="login.mail" label="邮箱" suffix="@gmail.com"></mu-text-field>
                        </mu-flex>
                    </div>
                </mu-col>
                <mu-col :span="3"></mu-col>
                <mu-col :span="12">
                    <mu-row gutter>
                        <mu-col :span="3"></mu-col>
                        <mu-col :span="6">
                            <mu-flex justify-content="center">
                                <mu-button full-width color="success" @click="lofi">登录</mu-button>
                            </mu-flex>
                        </mu-col>
                        <mu-col :span="3"></mu-col>
                        <mu-col :span="2"></mu-col>
                        <mu-col :span="8"></mu-col>
                        <mu-col :span="2">
                            <mu-flex justify-content="end">
                                <mu-button flat color="primary">没有账号？点击此处！</mu-button>
                            </mu-flex>
                        </mu-col>
                    </mu-row>
                </mu-col>
            </mu-row>
        </mu-container>
    </div>
</template>

<script>
export default {
    name: "login",
    data(){
        return{
            login:{
                username: "",
                password: "",
            },
            show: false,
            title: "登录",
            visibility: false
        }
    },
    methods:{
        lofi(){
            this.axios({
                method: "post",
                url: this.urls2+"login",
                data: JSON.stringify(this.login),
            }).then((res)=>{
                let loading = this.$loading();
                if(res.data.status){
                    switch(res.data.status){
                        case 250:
                            setTimeout(()=>{
                                loading.close();
                                this.$toast.info('服务器出错！');
                            },800);
                            break;
                            case 251:
                                setTimeout(()=>{
                                    loading.close();
                                    this.$toast.warning('用户不存在！');
                                },800);
                                break;
                                case 252:
                                    setTimeout(()=>{
                                        loading.close();
                                        this.$toast.error('密码出错！');
                                    },800);
                                    break;
                                    default:
                                        break;
                    }
                }else{
                    setTimeout(() => {
                        loading.close();
                        this.$toast.success("登录成功！");
                        window.localStorage.setItem("token",res.data.password);
                        window.localStorage.setItem("uuid",res.data.uuid);
                        window.localStorage.setItem("name",res.data.name);
                        window.localStorage.setItem("avatar",res.data.avatar);
                        this.$router.replace({name:"home"});
                    }, 800);
                };
            });
        }
    }
}
</script>

<style>
.login_main,.login_other {
    background-color: rgba(225, 225, 225, 0.7);
}
.login_main{
    border-radius: 100px 100px 0 0;
}
</style>