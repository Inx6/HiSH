<template>
    <div>
        <mu-carousel :active="active" style="height:50vh">
            <template slot="indicator" slot-scope="{ index, active }">
                <mu-button icon class="mu-carousel-indicator-button" :class="{'mu-carousel-indicator-button__active': active}" @click="changeActive(index)">
                <span class="rect-indicator"></span>
                </mu-button>
            </template>
            <div v-for="(hr,index) in src[0]" :key="index">
                <mu-carousel-item>
                    <img :src="hr.image" @click="advance(hr)">
                </mu-carousel-item>
            </div>
        </mu-carousel>
    </div>
</template>

<script>
// 公告栏
export default {
    name: "scroll-img",
    props:{
        current:{
            default: Array
        }
    },
    data(){
        return{
            src: [],
            active: 0
        }
    },
    mounted(){
        // console.log(this.current)
        this.src.push(this.current);
        // console.log(this.src)
    },
    methods:{
        changeActive (index) {
            this.active = index;
        },
        advance(e){
            const loading = this.$loading();
            setTimeout(() => {
                loading.close();
                this.$router.push({name: 'essay',params: e});
            }, 500);
        }
    }
}
</script>