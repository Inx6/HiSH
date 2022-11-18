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
                text: "观看内容",
                name: "essay",
                path: "/essay"
            }]
        }
    },
    methods:{
        jump(e){
            if(e.name !== this.$route.name){
                this.$router.push({path: e.path});
            }
        }
    }
}
</script>