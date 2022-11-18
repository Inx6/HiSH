<template>
    <div>
        <mu-row class="edits">
            <mu-col :span="1"></mu-col>
            <mu-col :span="10">
                <mu-flex justify-content="center">
                    <mu-text-field v-model="info.title" multi-line :rows="3" :max-length="50" label="标题" full-width></mu-text-field>
                </mu-flex>
            </mu-col>
            <mu-col :span="1"></mu-col>

            <mu-col :span="1"></mu-col>
            <mu-col :span="10">
                <mu-flex justify-content="center">
                    <mu-text-field v-model="info.briefly" multi-line :rows="3" :max-length="100" label="简介" full-width></mu-text-field>
                </mu-flex>
            </mu-col>
            <mu-col :span="1"></mu-col>

            <mu-col :span="1"></mu-col>
            <mu-col :span="10">
                <mu-flex justify-content="center">
                     <mu-text-field multi-line label="封面" disabled full-width></mu-text-field>
                    <input type="file" @change="upimg">
                </mu-flex>
            </mu-col>
            <mu-col :span="1"></mu-col>

            <mu-col :span="1"></mu-col>
            <mu-col :span="10">
                <mu-flex justify-content="end">
                    <mu-button v-if="submit_1" color="success" @click="semd">提交作品相关信息</mu-button>
                    <!-- <iframe src="//player.bilibili.com/player.html?aid=425768290&bvid=BV1a3411T73W&cid=580539054&page=1" scrolling="no" border="0" frameborder="no" framespacing="0" allowfullscreen="true"> </iframe> -->
                </mu-flex>
            </mu-col>
            <mu-col :span="1"></mu-col>
        </mu-row>
    </div>
</template>

<script>
export default {
    name: "infos",
    data(){
        return{
            info:{
                title: '',
                time: '',
                image: '',
                uuid: '',
                briefly: ''
            },
            submit_1: true
        }
    },
    methods:{
        upimg(e){
            let formdata = new FormData();
            formdata.append('file',e.currentTarget.files[0]);
            // formdata.append('dats',"85");

            this.axios({
                method: "post",
                url: this.urls3+"uploads",
                data: formdata,
            }).then((res)=>{
                let s = res.data.lastIndexOf("\\");
                console.log(s);
                // console.log(res.data.substring(s+1));
                this.info.image = this.urls2 +'image/'+ res.data.substring(s+1);
                console.log(this.info.image);
            })
        },
        semd(){
            this.matchs().then((_res)=>{
                // console.log(_res)
                this.info.uuid = window.localStorage.getItem("uuid");
                this.$emit("news_info",this.info);
            }).catch((res)=>{
                this.info.time = res;
                this.info.uuid = window.localStorage.getItem("uuid");
                this.$emit("news_info",this.info);
            });
        },
        matchs(){
            return new Promise((res,rej)=>{
                if(this.info.time !== ""){
                    res("ok");
                }else{
                    let myDate = new Date();
                    rej(myDate.toLocaleString());
                }
            })
        }
    }
}
</script>

<style>
.edits{
    background-color: white;
}
</style>