<template>
    <div>
        <mu-container class="demo-container">
            <mu-row justify-content="center" style="margin:0.5em">
                <mu-avatar size="85">
                    <img :src="src">
                </mu-avatar>
            </mu-row>
            <mu-row justify-content="center" style="color:white">
                <div>晚上好！{{name}}</div>
            </mu-row>
            
            <mu-container>
                <mu-row gutter>
                    <mu-col span="12">
                        <mu-paper :z-depth="5" style="background-color:rgba(225,225,225,0)">
                            <mu-list> 
                                <mu-list-item style="background-color:#2196f3;">
                                    <mu-list-item-content>
                                        <mu-list-item-title style="color:white;text-align:center" @click="open">
                                            <mu-ripple>
                                                <b>{{opmg}}</b>
                                            </mu-ripple>
                                        </mu-list-item-title>
                                    </mu-list-item-content>
                                </mu-list-item>
                                <mu-expand-transition>
                                    <mu-list v-show="opennings"> 
                                        <!-- <mu-divider shallow-inset></mu-divider>
                                        <mu-list-item>
                                            <mu-list-item-content>
                                                <mu-list-item-title>设置</mu-list-item-title>
                                            </mu-list-item-content>
                                        </mu-list-item> -->
                                        <mu-divider shallow-inset></mu-divider>
                                        <div @mousemove="changeColor" @mouseout="deleteColor">
                                            <mu-list-item>
                                                <mu-list-item-content>
                                                    <mu-list-item-title style="text-align:center" @click="publishs">发布文章</mu-list-item-title>
                                                </mu-list-item-content>
                                            </mu-list-item>
                                        </div>
                                        <mu-divider shallow-inset></mu-divider>
                                        <div @mousemove="changeColor" @mouseout="deleteColor">
                                            <mu-list-item>
                                                <mu-list-item-content>
                                                    <mu-list-item-title style="text-align:center" @click="exit">退出登录</mu-list-item-title>
                                                </mu-list-item-content>
                                            </mu-list-item>
                                        </div>
                                    </mu-list>
                                </mu-expand-transition>
                            </mu-list>
                        </mu-paper>
                    </mu-col>
                </mu-row>
            </mu-container>
        </mu-container>
    </div>
</template>

<script>
export default {
    name: "mylist",
    data(){
        return{
            opennings: false,
            opmg: "点击此处",
            name: window.localStorage.getItem("name"),
            src: window.localStorage.getItem("avatar"),
        }
    },
    methods:{
        open(){
            switch(this.opennings){
                case true:
                    this.opennings = false;
                    this.opmg = "打开";
                    break;
                    case false:
                        this.opennings = true;
                        this.opmg = "收起"
                        break;
                        default:
                            break;
            }
        },
        changeColor(e){
            let doms = e.currentTarget;
            doms.style.opacity = "1";
            doms.style.backgroundColor = "rgba(225,225,225,0.8)";
        },
        deleteColor(e){
            let doms = e.currentTarget;
            doms.style.opacity = "0.3";
        },
        publishs(){
            let loading = this.$loading();
            setTimeout(()=>{
                loading.close();
                this.$router.push({name: "publish",params:{"index":"0"}})
            },500);
        },
        exit(){
            window.localStorage.clear();
            this.$router.replace({name: "login"});
        }
    }
}
</script>

<style>

</style>