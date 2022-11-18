<template>
    <div>
        <v-header></v-header>
        <v-essay_header></v-essay_header>
        <v-boxw :msg="data_1" :border="bix" :title="title"></v-boxw>
        <v-comments :title_id="data_1" @revives="recontent"></v-comments>
        <v-remarks v-if="show" :information="remarks_information"></v-remarks>
    </div>
</template>

<script>
// 内容栏
import header from '../compen/header.vue';
import boxw from '../essay/box.vue';
import comments from '../essay/comments.vue';
import remarks from '../essay/remarks.vue';
import essay_header from '../essay/header.vue';

export default {
    name: 'essay',
    components:{
        'v-header': header,
        'v-boxw' : boxw,
        'v-comments': comments,
        'v-remarks': remarks,
        'v-essay_header': essay_header
    },
    data(){
        return{
            show: true,
            remarks_information: [
                // 模板评论内容格式
                // {
                //     id: "",
                //     name: "",
                //     avatar: "",
                //     comment: ""
                // }
            ],
            data_1: "",
            title: '',
            bix: ''
        }
    },
    mounted(){
        // console.log(this.$route.params);
        this.data_1 = this.$route.params.newid;
        this.title = this.$route.params.title;
        this.bix = this.$route.params.content;
        document.querySelector(".bod").innerHTML = this.bix;
    },
    methods:{
        recontent(e){
            console.log(e)
            if(e.uuid !== "" && e.uuid !== undefined && e.name !== "" && e.name !== undefined){
                if(e.comment !== "" && e.comment !== undefined && e.newid !== "" && e.newid !== undefined){
                    this.remarks_information.push(e);
                    this.axios({
                        method: "post",
                        url: this.urls2+"remark",
                        data: JSON.stringify({
                            newid: e.newid,
                            uuid:  e.uuid,
                            comment: e.comment
                        })
                    }).then((res)=>{
                        // console.log(res.data)
                        if(res.data.status === 200){
                            this.$toast.success('评论成功！');
                        };
                    });
                    this.refresh().then((es)=>{
                        this.show = false;
                        this.refresh().catch((ej)=>{
                            this.show = true;
                        });
                    })
                }else{
                    this.$toast.warning('您有信息被遗漏,请检查个人信息配置！');
                }
            }else{
                this.$toast.warning('您尚未完成登录！');
            }
        },
        refresh(){
            return new Promise((res,rej)=>{
                if(this.show == true){
                    res(1)
                }else{
                    rej(2)
                }
            })
        }
    }
}
</script>