import Vue from 'vue'
import Router from 'vue-router'
import home from '@/components/home'
import essay from '@/components/essay'
import publish from '@/components/publish'
import login from '@/components/login'

Vue.use(Router)

export default new Router({
  routes: [
    {
      path: '/',
      redirect: '/home'
    },{
      path: '/home',
      name: 'home',
      component: home,
      meta:{
        keepAlive: true
      }
    },{
      path: '/essay',
      name: 'essay',
      component: essay,
      meta:{
        keepAlive: false
      }
    },{
      path: '/publish',
      name: 'publish',
      component: publish,
      meta:{
        keepAlive: false
      }
    },{
      path: '/login',
      name: 'login',
      component: login,
      meta:{
        keepAlive: false
      }
    }
  ]
})
