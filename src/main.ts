import { createApp } from "vue";
import App from "./App.vue";
import './style.css'

const app = createApp(App);
app.mount("#app");

app.config.errorHandler = (err) => {
    console.error(`Application ERROR: ${err}`)
  }
