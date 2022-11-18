<template>
    <div>
        <mu-container style="background-color:white;">
            <mu-row gutter>
                <mu-col :span="12">
                    <mu-flex justify-content="start">
                        <h2>讨论区</h2>
                    </mu-flex>
                </mu-col>
                <mu-col :span="12">
                    <mu-row v-for="(item,index) in information" :key="index" style="margin-top:0.5em;line-height:1em">
                        <mu-col :span="1"></mu-col>
                        <mu-col :span="1">
                            <mu-flex justify-content="center">
                                <mu-avatar size="50">
                                    <img :src="item.avatar">
                                </mu-avatar>
                            </mu-flex>
                        </mu-col>
                        <mu-col :span="10">
                            <mu-row>
                                <mu-col :span="12">
                                    <p>{{item.name}}</p>
                                </mu-col>
                                <mu-col :span="12">
                                    <p :class="'text'+index">{{item.comment}}</p>
                                </mu-col>
                            </mu-row>
                            <mu-row>
                                <mu-col :span="12">
                                    <mu-expansion-panel>
                                        <div slot="header">回复评论</div>
                                        <mu-tooltip content="本条评论将采用加密技术发送，保卫您的隐私安全！">
                                            <mu-text-field placeholder="请输入..." solo full-width v-model="remark_user"></mu-text-field>
                                        </mu-tooltip>
                                        <mu-button slot="action" flat @click="lifted">取消</mu-button>
                                        <mu-button slot="action" flat color="primary" @click="send_comment">发送</mu-button>
                                    </mu-expansion-panel>
                                </mu-col>
                            </mu-row>
                        </mu-col>
                        <mu-divider></mu-divider>
                    </mu-row>
                </mu-col>
            </mu-row>
        </mu-container>
    </div>
</template>

<script>
export default {
    name: "remarks",
    props:{
        information:{
            type: Array
        }
    },
    data(){
        return{
            remark_user:"",
        }
    },
    mounted(){
        if(this.information.length > 0){
            this.information.forEach((i,e) => {
                // console.log(i,e)
                document.querySelector('.text'+e).innerHTML = i.comment;
            });
        }
    },
    methods:{
        // 发送回复评论
        send_comment(){
            console.log(this.remark_user);
        },
        //  取消评论回复
        lifted(){
            this.remark_user = ""
        }
    }
}
</script>