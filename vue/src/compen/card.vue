<template>
    <div>
        <mu-container>
            <mu-row gutter>
                <mu-col span="6" v-for="(item,index) in current" :key="index">
                    <mu-ripple>
                        <mu-paper :z-depth="5" style="margin:0.25em;background-color:rgba(225,225,225,0)">
                            <mu-container @click="openessay(item)">
                                <mu-card style="width: 100%; max-width: 80vw; margin: 0 auto;">
                                    <mu-card-header :title="item.name" :sub-title="item.time">
                                        <mu-avatar slot="avatar">
                                        <img :src="item.avatar">
                                        </mu-avatar>
                                    </mu-card-header>
                                    <mu-card-media>
                                        <img :src="item.image">
                                    </mu-card-media>
                                    <mu-card-title :title="item.title"></mu-card-title>
                                    <mu-card-text>
                                        {{item.briefly}}
                                    </mu-card-text>
                                    <mu-card-actions>
                                        <!-- <mu-button flat>Action 1</mu-button>
                                        <mu-button flat>Action 2</mu-button> -->
                                    </mu-card-actions>
                                </mu-card>
                            </mu-container>
                        </mu-paper>
                    </mu-ripple>
                </mu-col>
            </mu-row>
        </mu-container>
    </div>
</template>

<script>
// 文章简介栏
export default {
    name: 'cards',
    props:{
        current: Array
    },
    data(){
        return{
            src:[
                "https://muse-ui.org/img/uicon.ac3913bf.jpg",
            ]
        }
    },
    methods:{
        openessay(e){
            const loading = this.$loading();
            setTimeout(() => {
                loading.close();
                this.$router.push({name: 'essay',params: e});
            }, 500);
        }
    },
}
</script>