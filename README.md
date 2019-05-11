# Mastering Python Design Patterns, Second Edition, in Rust

This is the code repository for codes of [Mastering Python Design Patterns, Second Edition](https://www.packtpub.com/application-development/mastering-python-design-patterns-second-edition?utm_source=github&utm_medium=repository&utm_campaign=), published by Packt, rewriten in Rust.


## What Is This Repository for Exactly?

I am recently learning Rust and have noticed that Rust is really difficult to understand in terms of memory management and OOP concepts. So, I have decided to practice Rust by rewriting python codes in Rust.


## Instructions and Navigations

Refer to the original repository's [README.md](https://github.com/PacktPublishing/Mastering-Python-Design-Patterns-Second-Edition/blob/master/README.md).


## Progress

* Chapter 1
  * abstract_factory.py -> Done
  * factory_method.py -> Work in Progress
  * id.py
* Chapter 2
  * apple_factory.py -> Done
  * builder.py
  * computer_builder.py
  * exercise_fluent_builder.py
* Chapter 3
  * prototype.py
  * singletone.py
* Chapter 4
  * adapter.py
  * external.py
* Chapter 5
  * mymath.py
  * number_sum.py
  * number_sum_naive.py
* Chapter 6
  * bridge.py
* Chapter 7
  * facade.py
* Chapter 8
  * flyweight.py
  * lazy.py
  * mvc.py -> Done
  * proxy.py
* Chapter 9
  * chain.py
* Chapter 10
  * command.py
  * first-class.py
* Chapter 11
  * observer.py
* Chapter 12
  * state.py
* Chapter 13
  * boiler.py
  * interpreter.py
  * iterator.py
  * memento.py
  * strategy.py
  * template.py
* Chapter 14
  * people_list.py
  * rx_example1.py
  * rx_example2.py
  * rx_example3.py
  * rx_peoplelist1.py
  * rx_peoplelist2.py
  * rx_peoplelist3.py
* Chapter 15
  * ?.py (I don't yet understand how the files should be grouped)


## Contribution

Any suggestions and pull requests are welcome!

If you want to write Rust codes, folk this repository and create a pull request(one code per one pull request is ideal).

Before you start to contribute, please check the following pull request process:

1. Folk this repository.
2. Clone this repository to your local environment.
3. Move to the directory where the python code you want to rewrite exists.
4. Create a project with Cargo.
5. Move to the directory created in No.4, and create `.gitignore` containing the following entry:

```
/target/
```

6. Write code.
7. Update `Progress` section in `README.md`.
8. Create a pull request.

Note that it might be fine if, for example, you only rewrite the half of the target Python code, but make sure that there's something people can notice the progress(i.e. writing progress in the code as comment).


## License

This project is licensed under the MIT License - see the LICENSE file for details
