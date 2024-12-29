# Introduction

Supported Model Vendor:
- [x] OpenAI
- [x] Cohere
- [x] Google Gemini
- [x] xAI
- [x] Doubao


A quick demo of private RAG is shown below and make sure you have the API KEY.

```shell
$ git clone https://github.com/YAKNetwork/YAK
$ cd YAK
$ DOUBAO_API_KEY=$DOUBAO_API_KEY cargo run --bin rag --features=derive

Successfully loaded and chunked PDF documents
Successfully generated embeddings
Successfully created vector store and index
Starting CLI chatbot...
Welcome to the chatbot! Type 'exit' to quit.
> hello!
========================== Response ============================
Hi! Is there a specific question you have related to the provided documents? If so, please let me know and I'll do my best to answer.
================================================================


> brief summary the docs
========================== Response ============================
The provided documents cover different topics:
- **Moore's Law for Everything (parts 1 and 9)**
    - Discusses the need for a pivot towards the future, considering the radically different society that's approaching. It predicts an AI revolution that will be the fourth great technological revolution after the agricultural, industrial, and computational ones. This revolution could generate wealth for all if managed responsibly. It also mentions that technology will drive down the cost of goods, increasing societal wealth. The changes are unstoppable, and if embraced and planned for, they can lead to a fairer, happier, and more prosperous society.
- **Last Question (parts 6 and 13)**
    - In part 6, characters discuss concerns about the future as the Galaxy is expanding and will be filled in five years at the current rate. They consider submitting a report to the Galactic Council and debate the implications of immortality and the Galactic AC's solutions. Part 13 simply contains the phrase "there was light --", which provides little context on its own but may be part of a larger narrative.
================================================================


> quit
========================== Response ============================
Okay. If you have any other questions later, feel free to come back. Have a great day!
================================================================

```
