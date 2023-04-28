# <img src="icon.png" width="100" height="100" /> Broca

A ChatGPT powered dictionary (English <-> Chinese|Spanish|Japanese|Korean|German|French|Portuguese) + vocabulary book

https://user-images.githubusercontent.com/326807/233854967-a3ec9120-b6da-46e2-8aaf-b1bac308c877.mp4

## Introduction

Introducing Broca - the ultimate dictionary and vocabulary book app, powered by ChatGPT API! Have you ever struggled to keep track of English words you come across and want to learn, especially when using multiple devices? Look no further than Broca. This powerful app allows you to effortlessly organize and synchronize your vocabulary book across all of your devices.

With Broca, you can easily save words as plain JSON files in a folder, ensuring seamless synchronization across all of your devices. But that's not all. With ChatGPT as its dictionary engine, Broca is capable of searching for idioms, phrases, and slang that are not easily found in traditional dictionaries. Plus, it can generate unlimited example sentences to help you better understand the context and usage of each word.

And the name? Broca is named after the [brain region](https://www.hopkinsmedicine.org/news/media/releases/brocas_area_is_the_brains_scriptwriter_shaping_speech_study_finds) responsible for language, making it the perfect name for an app designed to help you master the English language. Download Broca today and start building your vocabulary like a pro!

## Installation

Prebuilt installers are available on the [release page](https://github.com/shi-yan/broca/releases) (MacOS only for now).

## Usage



## Build

1. Install [tauri-cli](https://tauri.app/v1/guides/getting-started/setup/html-css-js). I prefer using Cargo.
```bash
cargo install tauri-cli
```

2. Build
```bash
cd ./frontend
npm i
cd ../src-tauri
cargo tauri dev
```

## Todos
* Click to load more example sentences
* GPT Usage Gauge
* Linux / Win support
* Select Voice
