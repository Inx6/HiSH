<template>
    <div>
        <v-headers></v-headers>
        <v-breadcrumbs></v-breadcrumbs>
        <v-info @news_info="accept"></v-info>
        <v-editor v-if="show" :type="type" @serdes="serde"></v-editor>
    </div>
</template>

<script>
import editor from '../essay/wangeditor.vue';
import headers from '../compen/header.vue';
import publish_breadcrumbs from "../publish/header.vue";
import middle from '../publish/infos.vue';
export default {
    name: "publish",
    components:{
        'v-editor': editor,
        'v-headers': headers,
        'v-breadcrumbs': publish_breadcrumbs,
        'v-info': middle
    },
    data(){
        return{
            type: "publish",
            info: {
                newid: '',
                title: '',
                content: '',
                time: '',
                image: '',
                uuid: '',
                briefly: ''
            },
            show: false,
        }
    },
    methods:{
        serde(e){
            this.info.content = e;
            this.info.newid = "CT"+ Math.random().toString(36).slice(-10);;
            if(this.info.title !== "" && this.info.newid !== "" && this.info.content !== "" && this.info.time !== "" && this.info.image !== "" && this.info.briefly !== ""&& this.info.uuid !== ""){
                console.log(this.info);
                this.axios({
                    method: "post",
                    url: this.urls2+'upnews',
                    data: JSON.stringify(this.info)
                }).then((res)=>{
                    if(res.data.status === 200){
                        this.$toast.success('上传成功！');
                        this.$router.replace({name: "home"});
                    }
                })
            }else{
                this.$toast.warning('资料不完整！请认真检查！');
            }
        },
        accept(e){
            this.info.title = e.title;
            this.info.time = e.time;
            this.info.image = e.image;
            this.info.uuid = e.uuid;
            this.info.briefly = e.briefly;
            if(this.info.title !== "" && this.info.time !== "" && this.info.image !== "" && this.info.briefly !== ""&& this.info.uuid !== ""){
                this.show = true;
            }else{
                this.$toast.error('资料不完整！请认真检查！');
            };
        },
    }
}
</script>