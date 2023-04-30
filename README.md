# <img src="icon.png" width="100" height="100" /> Broca

A ChatGPT powered dictionary (English <-> Chinese|Spanish|Japanese|Korean|German|French|Portuguese) + vocabulary book

https://user-images.githubusercontent.com/326807/235324780-2cb41230-e330-4bc1-b3c8-f57c1088b639.mp4

## Introduction

Introducing Broca - the ultimate dictionary and vocabulary book app, powered by ChatGPT API! Have you ever struggled to keep track of English words you come across and want to learn, especially when using multiple devices? Look no further than Broca. This powerful app allows you to effortlessly organize and synchronize your vocabulary book across all of your devices.

With Broca, you can easily save words as plain JSON files in a folder, ensuring seamless synchronization across all of your devices. But that's not all. With ChatGPT as its dictionary engine, Broca is capable of searching for idioms, phrases, and slang that are not easily found in traditional dictionaries. Plus, it can generate unlimited example sentences to help you better understand the context and usage of each word.

And the name? Broca is named after the [brain region](https://www.hopkinsmedicine.org/news/media/releases/brocas_area_is_the_brains_scriptwriter_shaping_speech_study_finds) responsible for language, making it the perfect name for an app designed to help you master the English language. Download Broca today and start building your vocabulary like a pro!

## Installation

Prebuilt installers are available on the [release page](https://github.com/shi-yan/broca/releases) (MacOS only for now).

## Usage

<img width="912" alt="Screenshot 2023-04-27 at 9 45 07 PM" src="https://user-images.githubusercontent.com/326807/235056654-40f5b3f9-acbb-4600-9671-27c5b9bf3d63.png">

Firstly, create a folder to store your vocabulary book. All words will be saved to this folder, which can be found at <folder>/vocabulary. This will help you keep your vocabulary organized and easily accessible.

Secondly, it is mandatory to provide your OpenAI API key. This allows Broca to access the powerful language processing capabilities of OpenAI, which greatly enhances the app's performance and accuracy.

Thirdly, choose your target language (the language you want to translate to). At the moment, we support Chinese, Spanish, Japanese, Korean, German, French, and Portuguese.

Optionally, you can provide your AWS key and secret for pronunciation purposes. If you choose to do so, please ensure that your AWS key has full access to the AWS service Polly. This will enable Broca to accurately pronounce words for you, which can be a great help when learning a new language.

By following these simple steps, you'll be ready to start using Broca to expand your language skills and communicate with confidence.

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
* Linux / Win support
* Select Voice
* More Languages
