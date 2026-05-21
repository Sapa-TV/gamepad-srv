# Changelog
All notable changes to this project will be documented in this file. See [conventional commits](https://www.conventionalcommits.org/) for commit guidelines.

- - -
## [0.4.1](https://github.com/Sapa-TV/gamepad-srv/compare/cf2a2bfa51fc93e4314b8cf22140de0058400912..0.4.1) - 2026-05-21
#### Bug Fixes
- bump version - ([bab5434](https://github.com/Sapa-TV/gamepad-srv/commit/bab54347c4f9802c850e5c3cd7b1732e5fdbc6af)) - Th0rN13

- - -

## [0.4.0](https://github.com/Sapa-TV/gamepad-srv/compare/8eda9b0deaf008e862856701eb2d510b55454f1a..0.4.0) - 2026-05-21
#### Features
- add ws flow: worker, upgrade, update frontend - ([dda86c6](https://github.com/Sapa-TV/gamepad-srv/commit/dda86c6b4846d6104c764e2d224364780a828989)) - Th0rN13
- add current skin handler, refactoring - ([d603438](https://github.com/Sapa-TV/gamepad-srv/commit/d6034385641a1163acb5ef14f6e030d2ca527239)) - Th0rN13
- add gamepad state sender - ([3b9ea64](https://github.com/Sapa-TV/gamepad-srv/commit/3b9ea64623fa4cd7289a3b5b4dbb283888ba8948)) - Th0rN13
- add ws sender, refactor - ([6a75751](https://github.com/Sapa-TV/gamepad-srv/commit/6a75751bbe7d7220c92eb92bb95657cde19fe79d)) - Th0rN13
- add skin manager, update interfaces - ([f54c5e0](https://github.com/Sapa-TV/gamepad-srv/commit/f54c5e0549966d116dba733d82d4a53cfc07cbf8)) - Th0rN13
- add axum server - ([f5194ad](https://github.com/Sapa-TV/gamepad-srv/commit/f5194ad220b49e75842fef2ee8a38d5ab5b261f4)) - Th0rN13
- add gamepad state - ([44baed3](https://github.com/Sapa-TV/gamepad-srv/commit/44baed35f325d8f4c3e74ed1ea2b6ba658143eac)) - Th0rN13
- add input mapper - ([d3b96a0](https://github.com/Sapa-TV/gamepad-srv/commit/d3b96a0fdc646a4584dc6de253e79fcd2a88b58c)) - Th0rN13
- hold buttons, combo processing - ([af2b0a7](https://github.com/Sapa-TV/gamepad-srv/commit/af2b0a7b33009aca2db6de1ede56e590618f8213)) - Th0rN13
- start implement listener - ([4dada07](https://github.com/Sapa-TV/gamepad-srv/commit/4dada0775c048417c7eed0da054f7448d4156af1)) - Th0rN13
- add raw input worker, empty input listener - ([8f48975](https://github.com/Sapa-TV/gamepad-srv/commit/8f489753b30f3d97969f454a5f570c21554ce601)) - Th0rN13
- start another implementation - ([8f38670](https://github.com/Sapa-TV/gamepad-srv/commit/8f386709fc920bbc925cbfd12c1a627418eb4e7f)) - Th0rN13
#### Bug Fixes
- update config flow, remove old files - ([8140651](https://github.com/Sapa-TV/gamepad-srv/commit/81406514f957d84e3743f2cb8547f5a42883b052)) - Th0rN13
- resolve warnings - ([369d921](https://github.com/Sapa-TV/gamepad-srv/commit/369d92162ea1b7cc0735b243e4ffd804c677e17e)) - Th0rN13
#### Documentation
- update scheme plan - ([dc04879](https://github.com/Sapa-TV/gamepad-srv/commit/dc04879c30af44dcb376b3ca7ade13d151b88d97)) - Th0rN13
- update scheme plan - ([3872799](https://github.com/Sapa-TV/gamepad-srv/commit/3872799872de2600cc770dfc8e8ac7b59f7c02e1)) - Th0rN13
- update scheme plan - ([e6a1047](https://github.com/Sapa-TV/gamepad-srv/commit/e6a1047996456c23b890d5f863e0965759dd4d81)) - Th0rN13
- update todos and remove old plans - ([6a16409](https://github.com/Sapa-TV/gamepad-srv/commit/6a16409f1aa9edcda8e2176ee48c6836120d7149)) - Th0rN13
- update scheme plan - ([769e586](https://github.com/Sapa-TV/gamepad-srv/commit/769e586d589b2643d2be516091b2c9dd3404af5d)) - Th0rN13
- reset files, update scheme plan - ([3f2b044](https://github.com/Sapa-TV/gamepad-srv/commit/3f2b044bde3b8b305ff35867fced949f1535ba3b)) - Th0rN13
- add button names single source of truth - ([c8cb542](https://github.com/Sapa-TV/gamepad-srv/commit/c8cb542366a14b53a3a9fe852735f867a6442c9f)) - Th0rN13
- add refactoring plan - ([8eda9b0](https://github.com/Sapa-TV/gamepad-srv/commit/8eda9b0deaf008e862856701eb2d510b55454f1a)) - Th0rN13
#### Refactoring
- remove unused, update serialize - ([04832a6](https://github.com/Sapa-TV/gamepad-srv/commit/04832a6b553f55a6f8c629be8f1747d65caae031)) - Th0rN13
- validation use button names - ([302bbe5](https://github.com/Sapa-TV/gamepad-srv/commit/302bbe5646b298dba2a07da2120cfb5557e70918)) - Th0rN13
- use strum message - ([d8c1d15](https://github.com/Sapa-TV/gamepad-srv/commit/d8c1d1520f2e5629162be348dc4b1a8565c1279d)) - Th0rN13
- update gamepad input etc - ([a6e3405](https://github.com/Sapa-TV/gamepad-srv/commit/a6e34053a6c4dcf36683f33b9f6bad15bd907332)) - Th0rN13
- remove unused index - ([6aa3c71](https://github.com/Sapa-TV/gamepad-srv/commit/6aa3c710ded1b6a58b78482cacce1bd3c346f1c9)) - Th0rN13
- use match instead of unwrap - ([08665b0](https://github.com/Sapa-TV/gamepad-srv/commit/08665b0e21516a464a71af5faf35e7a809ed6af1)) - Th0rN13
- add constants - ([71cbf0a](https://github.com/Sapa-TV/gamepad-srv/commit/71cbf0a683d5b41330f5020a81b63322f4a8ce0a)) - Th0rN13
- separate domain and infra - ([a487c89](https://github.com/Sapa-TV/gamepad-srv/commit/a487c8966f2568415419caa2cc33cd26045b3e7e)) - Th0rN13
- create gamepad input - ([e98308e](https://github.com/Sapa-TV/gamepad-srv/commit/e98308ee16d55c3f796b52184e6e10105b572044)) - Th0rN13

- - -

## [0.3.3](https://github.com/Sapa-TV/gamepad-srv/compare/abec5dbc53ceff5757b46ecedc6dff5bc2769951..0.3.3) - 2026-05-14
#### Bug Fixes
- resolve skin switch state machine issue - ([da17834](https://github.com/Sapa-TV/gamepad-srv/commit/da17834c155e7ed30560d9538e4f5709801b85dd)) - Th0rN13
- resolve error handling issue - ([388a4c5](https://github.com/Sapa-TV/gamepad-srv/commit/388a4c5eaf3b866b750c704133884eb7cb4ad16f)) - Th0rN13
#### Documentation
- update plan - ([c1b5784](https://github.com/Sapa-TV/gamepad-srv/commit/c1b5784b8a17af9a0592256a16bda116e6a99a22)) - Th0rN13
- update v3 plan - ([420cc63](https://github.com/Sapa-TV/gamepad-srv/commit/420cc63bdef1acd5eb06d26b388138816f8658b7)) - Th0rN13
- add v3 refactoring plan - ([a297820](https://github.com/Sapa-TV/gamepad-srv/commit/a2978202169a695bbc3ad0a9c4f68bd87e39b3e1)) - Th0rN13
- update plan to actual - ([e887f74](https://github.com/Sapa-TV/gamepad-srv/commit/e887f74d84973c86670fbac3ac25ceb2b3e13aab)) - Th0rN13
- update plan - ([4e2362a](https://github.com/Sapa-TV/gamepad-srv/commit/4e2362a35c884039d21343e4030a84e0b220911b)) - Th0rN13
- update plan - ([9c67ba0](https://github.com/Sapa-TV/gamepad-srv/commit/9c67ba0cd5457ff88e55d222564f293605afe90f)) - Th0rN13
- update plan - ([0fa6771](https://github.com/Sapa-TV/gamepad-srv/commit/0fa67713f59965044d2872d1d2afe08ea4f48667)) - Th0rN13
- add step 10 in v2 plan - ([28be925](https://github.com/Sapa-TV/gamepad-srv/commit/28be925b71f2023ffdb95e429b828507c40d5d1d)) - Th0rN13
- add refactoring plan v2 - ([6b64e12](https://github.com/Sapa-TV/gamepad-srv/commit/6b64e12b98d7b9db4557e10bab200c532f36933e)) - Th0rN13
- update plan - prev step make it - ([04fda00](https://github.com/Sapa-TV/gamepad-srv/commit/04fda0030a08c0eb9b719279b8b4c3489d6b0c1b)) - Th0rN13
- main checked - ([11e5f26](https://github.com/Sapa-TV/gamepad-srv/commit/11e5f26eb30f34a6f7a4ba75673bd5173cf4519c)) - Th0rN13
- add refactoring plan - ([abec5db](https://github.com/Sapa-TV/gamepad-srv/commit/abec5dbc53ceff5757b46ecedc6dff5bc2769951)) - Th0rN13
#### Refactoring
- update state machine flow - ([dd360d2](https://github.com/Sapa-TV/gamepad-srv/commit/dd360d26144386c9c8968c0da9c2cc2c5b208480)) - Th0rN13
- use state methods - ([a4b9ecb](https://github.com/Sapa-TV/gamepad-srv/commit/a4b9ecb5bd6fab6c0a7b9a000fd3bbdb5a0f5268)) - Th0rN13
- add state methods - ([35a6a99](https://github.com/Sapa-TV/gamepad-srv/commit/35a6a99f2bef43098144a4a7b426176d467f51ea)) - Th0rN13
- add direction from button trait - ([8c8f877](https://github.com/Sapa-TV/gamepad-srv/commit/8c8f877a3a7ffd7232502328a9d09a06e7d4fa90)) - Th0rN13
- update imports to be more consistent - ([c8640db](https://github.com/Sapa-TV/gamepad-srv/commit/c8640dbb8b1cff7f6cef1054c0bee9edc00b171c)) - Th0rN13
- move inputs from tasks - ([4dc319b](https://github.com/Sapa-TV/gamepad-srv/commit/4dc319b81b5516e0c95e82c62f5958e51d3b033a)) - Th0rN13
- abstract app buttons - ([20cd32e](https://github.com/Sapa-TV/gamepad-srv/commit/20cd32e0cfe6c399713bbcc0778d32f4811a0fbe)) - Th0rN13
- remove cloning ws channel - ([db78b03](https://github.com/Sapa-TV/gamepad-srv/commit/db78b036d46df8d7863e6456843428654097a072)) - Th0rN13
- use config clone for save - ([e7c6b2d](https://github.com/Sapa-TV/gamepad-srv/commit/e7c6b2dae21101577a0c1b3681b53a607c5e1585)) - Th0rN13
- simpler save config pattern - ([363f07d](https://github.com/Sapa-TV/gamepad-srv/commit/363f07d43ecf43a4578d64250639f23c1c75bc7f)) - Th0rN13
- delete empty gamepad input file - ([d3558c1](https://github.com/Sapa-TV/gamepad-srv/commit/d3558c10d58642c3c5d6e9415a6ad540c546ade8)) - Th0rN13
- remove unused methods - ([fcb0ba0](https://github.com/Sapa-TV/gamepad-srv/commit/fcb0ba0b19861064884aaa890b362cc80a4d111a)) - Th0rN13
- remove unused index - ([322cdde](https://github.com/Sapa-TV/gamepad-srv/commit/322cdde07bee4ea49e9c4ddaef3fe31478a18370)) - Th0rN13
- implement skin manager - ([8d94aca](https://github.com/Sapa-TV/gamepad-srv/commit/8d94aca20ed8b808bce3b3dfab2982706cad2931)) - Th0rN13
- clear button action, tasks - ([4569799](https://github.com/Sapa-TV/gamepad-srv/commit/45697994e5471977fd8a2291771e652003b3ff43)) - Th0rN13
- optimize timer - ([65c149b](https://github.com/Sapa-TV/gamepad-srv/commit/65c149b453b070ef1bc89dce0fb7f634fa04d4ea)) - Th0rN13
- update tasks to use state machine - ([653c9ff](https://github.com/Sapa-TV/gamepad-srv/commit/653c9ff0bb8ef717599edb5c633515762cf744d8)) - Th0rN13
- implement machine handle - ([a9133d4](https://github.com/Sapa-TV/gamepad-srv/commit/a9133d49d1839144284141f023a48be43133d48c)) - Th0rN13
- delete re-export files - ([7de4d3e](https://github.com/Sapa-TV/gamepad-srv/commit/7de4d3e931935d1884caf7693abe1669fe3464fd)) - Th0rN13
- update button actions - ([032436c](https://github.com/Sapa-TV/gamepad-srv/commit/032436c3dd50a88a9430e30d91c36e6328b0adcb)) - Th0rN13
- update tasks - ([659e455](https://github.com/Sapa-TV/gamepad-srv/commit/659e455fe8026c4ce5ad6d0b31b3b726c7c249bb)) - Th0rN13
- update imports at app - ([443cf07](https://github.com/Sapa-TV/gamepad-srv/commit/443cf07176b1907b5ae45af655599529381fd1b4)) - Th0rN13
- update handlers - ([9ed66b7](https://github.com/Sapa-TV/gamepad-srv/commit/9ed66b7e6fbc8815de5a35263f057bc8bbf598af)) - Th0rN13
- move event processor - ([0a652c4](https://github.com/Sapa-TV/gamepad-srv/commit/0a652c49bd5b5c561379389317f193ad73bdd522)) - Th0rN13
- move config and skin switch commands - ([f1774eb](https://github.com/Sapa-TV/gamepad-srv/commit/f1774eb8987cbc90cd99a3b491b1ffe832e09096)) - Th0rN13
- move config - ([4fc5047](https://github.com/Sapa-TV/gamepad-srv/commit/4fc5047ddedeff1c3efcae1c1efaf9778cbe1e33)) - Th0rN13
- move websocket handler - ([dcedc0f](https://github.com/Sapa-TV/gamepad-srv/commit/dcedc0fe8cf0bdfa9f2bb71ea01e0b0061024252)) - Th0rN13
- move skin switch state - ([03153d4](https://github.com/Sapa-TV/gamepad-srv/commit/03153d4929bb04fdc97c56a35f95c12469995fd2)) - Th0rN13
- move skin manager - ([4960682](https://github.com/Sapa-TV/gamepad-srv/commit/4960682d73c1a9becbbe16835ba66945036816c1)) - Th0rN13
- move skin manager discovery - ([e017877](https://github.com/Sapa-TV/gamepad-srv/commit/e0178774e139a8d12f6d69d5fb03ba1eff908d0e)) - Th0rN13
- move gamepad state - ([43273f4](https://github.com/Sapa-TV/gamepad-srv/commit/43273f4036c6d9fc099d43982045cfdbafd68f36)) - Th0rN13
- create folder structure - ([645f6fb](https://github.com/Sapa-TV/gamepad-srv/commit/645f6fb28f2e6568e7f78e47628bdba896bb8cf8)) - Th0rN13

- - -

## [0.3.2](https://github.com/Sapa-TV/gamepad-srv/compare/77972c97e139266aa3562e1b0c1ab818568ba09a..0.3.2) - 2026-05-14
#### Bug Fixes
- resolve terminal colored text issue - ([77972c9](https://github.com/Sapa-TV/gamepad-srv/commit/77972c97e139266aa3562e1b0c1ab818568ba09a)) - Th0rN13

- - -

## [0.3.1](https://github.com/Sapa-TV/gamepad-srv/compare/8d75db75cc8f3f9cc4031ffdb7dcb7caa7cf2699..0.3.1) - 2026-05-14
#### Bug Fixes
- initial skin loading flow save to config, sort by dir name - ([8d75db7](https://github.com/Sapa-TV/gamepad-srv/commit/8d75db75cc8f3f9cc4031ffdb7dcb7caa7cf2699)) - Th0rN13

- - -

## [0.3.0](https://github.com/Sapa-TV/gamepad-srv/compare/b1c9eab9c4ee0aefdfc5f252a88a7068a2124d05..0.3.0) - 2026-05-13
#### Features
- add config file using - ([fa64599](https://github.com/Sapa-TV/gamepad-srv/commit/fa645999bb8912a9b5fc9ad7de107e32ee86af78)) - Th0rN13
- add new skin, refactor skins - ([ce2d1d0](https://github.com/Sapa-TV/gamepad-srv/commit/ce2d1d006ef6281690cba4089121ea59ca0678dc)) - Th0rN13
#### Documentation
- add refactoring plan - ([f3474c2](https://github.com/Sapa-TV/gamepad-srv/commit/f3474c217506cd252a856b700661ad62b50ccf10)) - Th0rN13
- add readme.md - ([b1c9eab](https://github.com/Sapa-TV/gamepad-srv/commit/b1c9eab9c4ee0aefdfc5f252a88a7068a2124d05)) - Th0rN13

- - -

## [0.2.0](https://github.com/Sapa-TV/gamepad-srv/compare/836a1c4db57370ddaf4d238a984659137bbb78d7..0.2.0) - 2026-05-12
#### Features
- add single blink when skin switch ready - ([30f1050](https://github.com/Sapa-TV/gamepad-srv/commit/30f10506779f6b0d1ec53e9c8c5cad17c58f9075)) - Th0rN13
- skin switch flow fully work - ([6511e9b](https://github.com/Sapa-TV/gamepad-srv/commit/6511e9b759209b9830dd633e375d1a1b2cefa039)) - Th0rN13
- add skin switch state flow - ([3a1f951](https://github.com/Sapa-TV/gamepad-srv/commit/3a1f95186e854740b66422d7a8106af8782da20f)) - Th0rN13
- add skin changing blink animation - ([0dbffeb](https://github.com/Sapa-TV/gamepad-srv/commit/0dbffebd989c26fd6093748443d3d8b7f3a1180d)) - Th0rN13
- add timer module - ([6dc89c9](https://github.com/Sapa-TV/gamepad-srv/commit/6dc89c9f23f19bb6e2978bb492528c6c824f5e5a)) - Th0rN13
- add common event bus - broadcaster - ([a725782](https://github.com/Sapa-TV/gamepad-srv/commit/a72578230236ca07e32333eda863e05cac354626)) - Th0rN13
- add button actions - ([d027587](https://github.com/Sapa-TV/gamepad-srv/commit/d02758751234e06f2ccac29f0ee19e15ec0e972a)) - Th0rN13
- add skins validation and skins list - ([9214cc7](https://github.com/Sapa-TV/gamepad-srv/commit/9214cc7e5203dd976fd2df6411ca43f5c58a8bbc)) - Th0rN13
- validate skins, move to skin module - ([a42f8f3](https://github.com/Sapa-TV/gamepad-srv/commit/a42f8f3814e4e715010c764481810cf0188b3bfd)) - Th0rN13
- add backend skin flow - ([dd7ae16](https://github.com/Sapa-TV/gamepad-srv/commit/dd7ae16e42787d8ff6a9e4676633d225464f8375)) - Th0rN13
- skins loading flow - ([d9e0c7a](https://github.com/Sapa-TV/gamepad-srv/commit/d9e0c7a6270b2340fe83a69a6817fd61f6ab74e5)) - Th0rN13
- add skins files - ([836a1c4](https://github.com/Sapa-TV/gamepad-srv/commit/836a1c4db57370ddaf4d238a984659137bbb78d7)) - Th0rN13
#### Refactoring
- remove old files - ([83eaa11](https://github.com/Sapa-TV/gamepad-srv/commit/83eaa116e96394863bd13fb864c4ac2ae4955776)) - Th0rN13
- separate to different modules - ([1150015](https://github.com/Sapa-TV/gamepad-srv/commit/1150015388d495429f441fd47157741d1ffbf3df)) - Th0rN13

- - -

## [0.1.0](https://github.com/Sapa-TV/gamepad-srv/compare/7230cebe1f99a1fcd2fcf4bc632b463b6f872b0d..0.1.0) - 2026-04-24
#### Features
- move to new png assets - ([6754c0d](https://github.com/Sapa-TV/gamepad-srv/commit/6754c0d6666d836a930cc6fb692f339a332afa57)) - Th0rN13
- add js reconnect flow - ([5190fd7](https://github.com/Sapa-TV/gamepad-srv/commit/5190fd74b6203e2e12f076206fa67867d06f5880)) - Th0rN13
- add logging - ([6aa40d4](https://github.com/Sapa-TV/gamepad-srv/commit/6aa40d48bbac026c4c6dcabbeb0f7637a243de92)) - Th0rN13
- initial release - ([7230ceb](https://github.com/Sapa-TV/gamepad-srv/commit/7230cebe1f99a1fcd2fcf4bc632b463b6f872b0d)) - Maksim 'Th0rN13' Tolkachev
#### Bug Fixes
- sticks offset update, update image artefacts - ([2f8739d](https://github.com/Sapa-TV/gamepad-srv/commit/2f8739d560b4027e9de82cfe941d982c40216ce8)) - Th0rN13
- resolve server ws close connection only on exit - ([7a173bd](https://github.com/Sapa-TV/gamepad-srv/commit/7a173bd7bc836709040d5afb496b2057897d6610)) - Th0rN13
- change ws connetion status to gamepad paw button - ([86dbb49](https://github.com/Sapa-TV/gamepad-srv/commit/86dbb49e8638c8fcfff2e00f4d1c138e75677162)) - Th0rN13
- resolve stick moving issue - ([63bce5a](https://github.com/Sapa-TV/gamepad-srv/commit/63bce5a58409bf5c289f8ee303d81ba288543f79)) - Th0rN13
- move to gilrs crate, sticks press issue - ([92d09a5](https://github.com/Sapa-TV/gamepad-srv/commit/92d09a50b40ab35e9ef3fd55ce8a43caf303079e)) - Th0rN13
#### Documentation
- add todo.md - ([5986e59](https://github.com/Sapa-TV/gamepad-srv/commit/5986e5975cf44cbc69242645f4f1c54058939730)) - Th0rN13
#### Refactoring
- move to event model workflow - ([87f78e3](https://github.com/Sapa-TV/gamepad-srv/commit/87f78e336375e46b94c6a5b054b1c1eed572aaf2)) - Th0rN13

- - -

Changelog generated by [cocogitto](https://github.com/cocogitto/cocogitto).