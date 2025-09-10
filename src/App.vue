<script setup>
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { stat } from '@tauri-apps/plugin-fs'
import { emit } from '@tauri-apps/api/event';
import { resolveResource } from '@tauri-apps/api/path';
import { readTextFile } from '@tauri-apps/plugin-fs';
import { ask,open,save } from '@tauri-apps/plugin-dialog';
import Database from '@tauri-apps/plugin-sql';
import { Store,load } from '@tauri-apps/plugin-store';

const greetMsg = ref("");
const name = ref("");
let store = undefined;



async function greet() {
  // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
  greetMsg.value = await invoke("greet", { name: name.value });

  invoke("my_custom_command", { invoke_message: 'Hello!'}).then((message) => {
    console.log("response: " + message);
  });

  // await stat('C:\\opt\\upFiles\\temp\\IMG_1507_1733279200197.jpeg').then((resp) => {
  //   console.log(resp);
  // });

  // invoke("read_file").then((response) => {
  //   console.log(response);
  // });

  invoke("login", { user: "tauri", password: "oj4rijw8="}) 
    .then((message) => {
      console.log(message);
    })
    .catch((error) => {
      console.log(error);
    });

  invoke('my_webview_label');
  
  invoke("my_custom_command2", { number: 42 })
    .then((res) => {
      console.log(`Message: ${res.message}, Other Value: ${res.other_val}`)
    })
    .catch((e) => console.error(e));
  
  emit('ready', '/path/to/file');
  invoke("hello").then((res) => {
    console.log(res);
  })
  .catch((e) => console.log(e));

  const resourcePath = await resolveResource('lang/de.json');
  const langDe = JSON.parse(await readTextFile(resourcePath));
  console.log(langDe.hello); // 这里将在 devtools 的控制台中输出 'Guten Tag!'

  invoke('get_app_data').then((res) => {
    console.log(res);
  });

  invoke("get_app_state").then((res) => {
    console.log(res);
  });

  invoke("increase_counter").then((res) => {
    console.log(res);
  })
}

async function openDialog() {
  // // Create a Yes/No dialog
  // const answer = await ask('This action cannot be reverted. Are you sure?', {
  //   title: 'Tauri',
  //   kind: 'warning',
  // });

  // console.log(answer);

  // Open a dialog
  // const file = await open({
  //   multiple: false,
  //   directory: false,
  // });
  // console.log(file);
  // Prompt to save a 'My Filter' with extension .png or .jpeg
  // const path = await save({
  //   filters: [
  //     {
  //       name: 'My Filter',
  //       extensions: ['png', 'jpeg'],
  //     },
  //   ],
  // });
  // console.log(path);

  // 
  queryDb().then((resp) => {
    console.log(resp);
  });


  store = await load('settings.json', {});

  // 设置一个值。
  await store.set('some-key', { value: 5 });

  // 获取一个值。
  const val = await store.get('some-key');
  console.log(val); // { value: 5 }

  // 您可以在进行更改后手动保存存储
  // 否则如上所述，它将在正常退出时保存。
  await store.save();

}

// 初始化数据库
async function initDb() {
  const db = await Database.load('mysql://root:123456@192.168.31.105/navi_cloud_sinognss?useUnicode=true&characterEncoding=utf8&zeroDateTimeBehavior=convertToNull&useSSL=true&serverTimezone=GMT%2B8');

  return db;
}

// 查询数据
async function queryDb() {
  const db = await initDb();
  return await db.select("select * from data_gnss_202509");
}

async function openMqtt() {
  invoke("connect_mqtt");
}
</script>

<template>
  <main class="container">
    <h1>Welcome to Tauri + Vue</h1>

    <div class="row">
      <a>
        <img src="/vite.svg" class="logo vite" alt="Vite logo" />
      </a>
      <a>
        <img src="/tauri.svg" class="logo tauri" alt="Tauri logo" />
      </a>
      <a>
        <img src="./assets/vue.svg" class="logo vue" alt="Vue logo" />
      </a>
    </div>
    <p>Click on the Tauri, Vite, and Vue logos to learn more.</p>

    <form class="row"  size="small">
      <el-space wrap :size="10">
        <el-input id="greet-input" 
          v-model="name" 
          style="width:200px;"
          placeholder="Enter a name..." />
        <el-button type="primary" size="small" @click="greet">Greet</el-button>
      </el-space>
    </form>
    <el-button type="primary" size="small" style="width:200px;" @click="openDialog">Dialog</el-button><br/><br/>
    <el-button type="primary" size="small" style="width:200px;" @click="openMqtt">OpenMqtt</el-button>
    <p>{{ greetMsg }}</p>
  </main>
</template>

<style scoped>
.logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #249b73);
}

</style>
<style>
:root {
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  font-size: 16px;
  line-height: 24px;
  font-weight: 400;

  color: #0f0f0f;
  background-color: #f6f6f6;

  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
}

.container {
  margin: 0;
  padding-top: 10vh;
  display: flex;
  flex-direction: column;
  justify-content: center;
  text-align: center;
}

.logo {
  height: 6em;
  padding: 1.5em;
  will-change: filter;
  transition: 0.75s;
}

.logo.tauri:hover {
  filter: drop-shadow(0 0 2em #24c8db);
}

.row {
  display: flex;
  justify-content: center;
}

a {
  font-weight: 500;
  color: #646cff;
  text-decoration: inherit;
}

a:hover {
  color: #535bf2;
}

h1 {
  text-align: center;
}

button {
  cursor: pointer;
}

button:hover {
  border-color: #396cd8;
}
button:active {
  border-color: #396cd8;
  background-color: #e8e8e8;
}

input,
button {
  outline: none;
}

@media (prefers-color-scheme: dark) {
  :root {
    color: #f6f6f6;
    background-color: #2f2f2f;
  }

  a:hover {
    color: #24c8db;
  }

  input,
  button {
    color: #ffffff;
    background-color: #0f0f0f98;
  }
  button:active {
    background-color: #0f0f0f69;
  }
}

</style>
