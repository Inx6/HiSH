<template>
    <div>
        <mu-container style="background-color:white;">
            <mu-row gutter>
                <mu-col :span="12" style="margin-top:1em">
                    <mu-flex justify-content="end">
                        <mu-row gutter>
                            <mu-col :span="4" @mousemove="show1(1,$event)" @mouseout="show1(2,$event)">点赞</mu-col>
                            <mu-col :span="4" @mousemove="show1(1,$event)" @mouseout="show1(2,$event)">收藏</mu-col>
                            <mu-col :span="4" @mousemove="show1(1,$event)" @mouseout="show1(2,$event)">举报</mu-col>
                        </mu-row>
                    </mu-flex>
                </mu-col>
                <mu-col :span="12" style="margin-top:1em">
                    <mu-flex justify-content="start">
                        <h2>评论</h2>
                    </mu-flex>
                </mu-col>
                <mu-col :span="12">
                    <v-editor :type="types" @serdes="serde"></v-editor>
                </mu-col>
            </mu-row>
        </mu-container>
    </div>
</template>

<script>
import editors from './wangeditor.vue';
export default {
    name: 'comments',
    components:{
        'v-editor': editors
    },
    props:{
        title_id: String
    },
    data(){
        return{
            types: "watch"
        }
    },
    methods:{
        serde(e){
            this.$emit("revives",{
                newid: this.title_id,
                uuid: window.localStorage.getItem("uuid"),
                name: window.localStorage.getItem("name"),
                avatar: window.localStorage.getItem("avatar"),
                comment: e
            });
        },
        show1(n,e){
            switch (n){
                case 1:
                    e.target.style.color = "red";
                    e.target.style.fontFamily = '"Times New Roman"';
                    break;
                    case 2:
                        e.target.style.color = "";
                        e.target.style.fontFamily = '"Lucida Calligraphy", cursive, serif, sans-serif';
                        break;
                        default:
                            break;
            }
        }
    }
}
</script>