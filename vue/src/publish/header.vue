<template>
    <div>
        <mu-breadcrumbs style="background-color:white">
            <mu-breadcrumbs-item v-for="(item,index) in items" :key="index" :disabled="item.disabled">
                <p @click="jump(item)">{{item.text}}</p>
            </mu-breadcrumbs-item>
        </mu-breadcrumbs>
    </div>
</template>
<script>
export default {
    name: "publish_header",
    data(){
        return{
            items:[{
                text: "主页",
                name: "home",
                path: "/"

            },{
                text: "上传文章",
                name: "publish",
                path: "/publish"
            }]
        }
    },
    methods:{
        jump(e){
            if(e.name !== this.$route.name){
                this.$confirm('离开后将清除所有输入内容！', '警告⚠').then((res)=>{
                    switch(res.result){
                        case true:
                            this.$router.push({path: e.path});
                            break;
                            case false:
                                break;
                                default:
                                    break;
                    }
                    
                });
            }
        }
    }
}
</script>