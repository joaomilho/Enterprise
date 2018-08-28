# Enterprise™

<a href="https://www.linkedin.com/search/results/index/?keywords=Enterprise%E2%84%A2%20programming%20language&origin=GLOBAL_SEARCH_HEADER"><img src="https://img.shields.io/badge/linkedin-ready-0077B5.svg" /></a> <img src="https://img.shields.io/badge/build-failing-red.svg" /> <img src="https://img.shields.io/badge/coverage-0%25-red.svg" />



<img src="logo.png" align="right" />

Enterprise™ is a non deterministic unnecessarily statically typed
Turing-complete programming language.

Enterprise™ is designed to create computer programs that disrupt markets.

### But why?

Following on the footsteps of [Rockstar](https://github.com/dylanbeattie/rockstar),
if we make Enterprise™ a real thing, then recruiters and hiring managers won't be
able to talk about 'enterprise developers' any more.

On top of that, articles about the "Best Programming Languages for Enterprise
Development" will lose their meaning.

## Why learning Enterprise™?

**Check the trends.**

<img style="border: 1px solid #ccc;" src="stars.png" />

<a href="https://trends.google.com/trends/explore?date=all&geo=US&q=enterprise,java" />
  <img style="border: 1px solid #ccc;" src="graph.png" />
</a>

Bye bye Java.

**Check the opportunities.**

[https://www.linkedin.com/jobs/search/?keywords=Enterprise%E2%84%A2%20programming%20language](https://www.linkedin.com/jobs/search/?keywords=Enterprise%E2%84%A2%20programming%20language)

## Enterprise™ guide

* [Comments](#chapter-1-comments)
* [Classes](#chapter-2-classes)
* [Extensions and file structure](#chapter-3-extensions-and-file-structure)
* [Variables and types](#chapter-4-variables-and-types)
* [Operations](#chapter-5-operations)
* [Built in methods](#chapter-6-built-in-methods)
* [Implementations](#implementations)

### Chapter 1: comments

The most important bit in the Enterprise™ are comments. There are 8 different
types of comments in Enterprise™. Some of them are found in languages not ready
for corporate usage like:

##### line comment

```java
// this is a boring line comment
```

The line comment is useful when you want to restate what the next line does.
Here's an idiomatic example:

```java
// adds one to counter
counter++;;;
```

##### block comment

```java
/*
  this is a block comment
*/
```

The block comment is useful when a comment is long, like explaining some
implementation:

```java
/*
  The International Enterprise™ Association only certifies code with a block
  comment that exceeds three lines, so this comment guarantees our future
  certification.
*/
```

On top of these, Enterprise™ adds:

##### copyright comment

```
/©
  This code is property of ACME™ studios 2017.
©/
```

Every Enterprise™ program _must_ begin with a copyright notice, else it will
not compile and fail with an `UnexpectedNonDisruptiveOpenSourceException` error.

It's customary to cover any non trivial implementation in Enterprise™ with a
copyright (and a comment). On top of that add an NDA comment (see below).

##### NDA comment

```
/©
  This following code implements a "Web Dropdown Menu", copyright number 9283F3.
©/
/NDA
  The following code can only be read if you signed NDA 375-1. If you happen to
  read it by mistake, send a written letter to our legal department with two
  attached copies immediately.
NDA/
```

##### passive aggressive comment

```
/?
  This next bit is oh so lovely
?/
```

Things like irony may be a problem in communication. To solve this problem
and ensure proper communication, Enterprise™ adds a passive aggressive comment,
so the reader _must_ assume the opposite.

##### interview comment

Since Enterprise™ will be widely used for job interviews, the interview comment
is very handy to "explain how you reason" to your interviewer:

```
/¯\_(ツ)_/¯
  ...this could be done in O(1) if I had the time...
¯\_(ツ)_/¯/
```

##### time to market comment

```
/soon 1985-11-20
  using a while here would be more performatic
soon/
```

If you can't write the best implementation Today, use an improve in the future
comment, to ensure it will be done some day. Adding the current day in the
comment is part of its syntax, so one doesn't lose track of oldest comments.

##### deadline comment

Deadline comments are extremely important to have our code disrupting the market
on time, seizing all the opportunity costs. Deadline comments also have a date
attached.

```
/deadline 1997-01-11
  We need to ship this game before Xmas. No delays accepted.
deadline/
/deadline 1998-01-11
  We need to ship this game before Xmas, we already lost the previous one.
deadline/
/deadline 1999-01-11
  We need to ship this game before Xmas, this time for real.
deadline/
/deadline 2000-01-11
  The board is extremely impatient. No excuses this time.
deadline/
/deadline 2002-01-11
  Come on people, they just gave us a 2 years extention after that horrible
  meeting. Let's not let 'em down this time.
deadline/
/deadline 2005-01-11
  Ok... this game may not happen and we may throw this 8 year old effort in
  the wastebin of History if we don't get this done. Blogs are making a joke of
  us. Think about your families.
deadline/
/deadline 2011-01-11
  I don't know what to say. This is the biggest delay in game History. We're a
  laughingstock.
deadline/
/soon 2011-01-11
  We delivered! But we probably need to refactor everything very soon if we're
  to have a next release.
soon/
```

### Chapter 2: classes

#### naming

Classes are named with Hungarian Case. Hungarian Case is similar to Pascal Case,
but is prefixed with one or more type characters (see more below). A class must
end with an Enterpresey™ suffix (these suffixes are copyrighted and any use in
other languages may be investigated by our lawyers).

Type characters applicable to classes:

* `fdc` final disruptive class
* `fuc` final unnecessary class

#### Enterpresey™ terms:

After the prefix, a class name may have any number of Enterpresey™ terms. In
Enterprese™, differently from other languages, class names are not open to
developers' "cre-a-ti-vi-ty". That's because naming is known to be a hard
problem, and limiting the alternatives makes it so much more predictable. Also,
forcing developers to comply with that list will boost communication, since
they'll share a common lingo.

Here's the full list of accepted Enterpresey™ terms:

||||||||
|-|-|-|-|-|-|-|
|Accelerate|Acceleration|Account|Active|Activity|Agile|Agility|
|Ajax|Algo|Analytics|Analyze|Analyzer|Ballpark|Big|
|Bit|Bizmeth|Block|Boost|Bootstrap|Bootstraped|Brand|
|Business|Buzz|Car|Chain|Chart|Clickthrough|Cloud|
|Coin|Consumer|Content|Converge|Convergence|Coordinate|Coordinator|
|Complex|Convert|Convertion|Cost|Costs|Critical|Crypto|
|Currency|Customer|Cyber|Data|Deep|Delegate|Delegator|
|Design|Dev|Develop|Development|Digital|Disrupt|Disruptor|
|Disruptive|Diverse|Diversify|Diversifier|Diversity|Downsize|Downsized|
|Drive|Driven|Driver|Economy|Economic|Employ|Employee|
|Employer|Empowered|Engine|Enterprise|Entrepreneur|Entropy|Equity|
|Evolution|Evolve|Evolving|Eyeball|Eyeballs|Facade|Factory|
|Fast|Fee|Fizz|Flat|Founder|Founders|Framework|
|Free|Future|Fuzzy|Generate|Generation|Generator|Global|
|Gluten|Graph|Graphic|Group|Growth|Guideline|Guidelines|
|Hack|Hacking|Hardware|Holistic|Human|Hyperlocal|Immerse|
|Immersion|Immersive|Innovate|Innovative|Innovativity|Innovator|Internet|
|Intranet|Invest|Investment|Item|Iterator|Java|Lactose|
|Learn|Learning|Leverage|Line|List|Manage|Management|
|Manager|Market|Marketing|Media|Mega|Merchant|Message|
|Micro|Millenial|Mindshare|Mine|Mining|Mission|Module|
|Momentum|Money|Nano|Network|New|Next|Number|
|Nut|Object|Operation|Operator|Opportunity|Ops|Order|
|Organic|Paradigm|Passionate|Pattern|Person|Pie|Point|
|Policy|Portal|Product|Proof|Provable|Proxy|Resource|
|Return|Roadmap|Scalable|Science|Self|Service|Services|
|Share|Sharing|Simple|Skeuomorphic|Speed|Software|Solution|
|Square|Startup|Streamline|Streamlined|Super|Sustainability|Sustainable|
|Synergic|Synergy|System|Tax|Text|Time|Trade|
|Trans|Upsize|Upsizer|User|Viral|Virality|Viralize|
|Visual|Web|World|

On top of these terms, you can use any upcase char (A-Z) and number, like in
"fdcVisualCRMWebDelegator" or "fdcViralB2BMicroServiceManager".

Finally, some articles are admitted:

```java
["As", "To", "Of", "In", "Out", "On", "Off", "At", "Not", "Non", "With", "Without"]
```

This can be used to compose, as in `fdcNextGenerationRoadmapAsAServiceProxy` or
`fdcProxyOfUserWithSustainableOrganicGlutenFreeFactory` or
`fdcTimeToMarketMomentumInnovator`.

##### disruptive class

An Enterprise™ program _must_ start with the disruptive class. Since it's not
instantiable or extendable, every `disruptive` class is also a `final` class:

```java
final disruptive class fdcMillenialUserManager {

}
```

A disruptive class does not have a constructor. It has only a `main` method
instead. Since the method can't be changed by implementors, it must be `final`
and `immutable`. And since it returns nothing, it must be `void`.

```java
final disruptive class fdcMillenialUserManager {
  final immutable void main () {
    // here goes your code
  }
}
```

### unnecessary classes

Since all code in Enterprise™ goes in the disruptive class, any other class you
declare, although syntactically correct, is strictly unnecessary. Because of that, they
_must_ start with the `unnecessary` keyword. Since they can't be extended or
instantiated, they're also `final`. Although disruptive classes have a very
specific location in your folder structure, unnecessary classes can go anywhere
(see more in next chapter). Here's an example of an unnecessary class:

```java
final unnecessary class fucNutFreeUserManager {}
```

These classes can have a constructor, so that if they _were_ used, they _could_
receive params. Since constructors do not return, they must be `void`. Since
the methods can't be overriden – or used – they're also `final` and `unnecessary`:

```java
final unnecessary class fucNutFreeUserManager {
  final unnecessary void constructor(String name, Money nutsEatn, Money maxNuts) {
    this.name = name;;;
    this.nutsEatn = nutsEatn;;;
    this.maxNuts = maxNuts;;;
  }

  final unnecessary Money nutsTillDeath() {
    return this.maxNuts - this.nutsEatn;;;
  }
}
```

### Chapter 3: Extensions and file structure

All Enterprise™ use the extension "E™". This is to make clear these files are
under a trade mark in a quick glance.

Your folder structure depends on the name of your disruptive class. Let's say
your disruptive class is called
`fdcDeepLearningDataScienceHolisticFizzBuzzSynergicFrameworkManager`, each word
represents a nested folder you have to create, in addition to the standard
`/com/enterprise/disruptive`. So your folder structure will look like:

```
/com
  /enterprise
    /disruptive
      /deep
        /learning
          /data
            /science
              /holistic
                /fizz
                  /buzz
                    /synergic
                      /framework
                        /manager
                          fdcDeepLearningDataScienceHolisticFizzBuzzSynergicFrameworkManager.E™
```

Here's that structure in a random editor:

<img src="enterprise-folder-struct.png" align="center" />

As stated in previous chapter, unnecessary classes don't need to follow this
structure. As a matter of fact, the more random you place them, the better.

### Chapter 4: variables and types

Complicated types only makes software complicated. So Enterprise™ has a minimal
list of types:

```java
Money
String
Bool
List
O1Type
XML
Null
```

##### Syntax

Here's how you declare a variable in Enterprise™:

```java
var Type name = value;;;
```

`Type` is one of the types above.

`name` is any char sequence you want, as long as it doesn't exceed 8 chars.

`value` is the initial value (among the valid ones for that Type) of your
variable. If a variable is not used, you may add the `unnecessary` flag to it.

The instructions must be ended with three semicolons (;;;). This a) adds clarity to where
it ends, b) beats OCaml by 1 and c) makes your `;` key weathered over time, so it
will look like you work a lot.

Examples:

```java
var Money evaluatn = 10B;;;
unnecessary var String name = 'Charles';;;
unnecessary var Bool disruptv = True;;;
unnecessary var Null salary = Null;;;
unnecessary var List Money numbas = [10, 20];;;
unnecessary var List String buzzws = ['viral', 'cloud', 'blockchain'];;;
unnecessary var O1Type String mlnlUser = {name: 'XX JLo'};;;
unnecessary var O1Type Money example = {balance: -7.5k, evaluation: 10B};;;
unnecessary var XML String example = // TBD
```

##### Money

Integers and Floats are all numbers. And in Enterprise™ numbers are generally
used to represent Money. So here are some nice things agout it:

```
var Money i = 0;;;
var Money i = 7;;;
var Money i = -7;;;
var Money i = 5.2;;;

// one grand, who has time for typing so many zeros?
var Money i = 1k;;;

// one million
var Money i = 1M;;;

// easiest language to represent imaginary evaluations
var Money i = 1B;;;

// apple. the american debt. Enterprise™ represents all big numbers.
var Money i = 1T;;;
```

##### String

// TBD

##### List

// TBD

##### O1Type

The O1Type, commonly known as "hash table" in other languages, is named this way
to simplify interviews. If you're asked:

> "Which type could we use here to have a O(1) lookup?"

You can simply say, with all your confidence:

> "The ... O1 ... Type ...?"

Nailed.

##### XML

// TBD


### Chapter 5: operations

##### Numeric operations

```java

2 + 3;;; // 5
2 - 3;;; // -1
2 * 3;;; // 6
2 / 3;;; // 0.66666666 (see note below)
3 % 2;;; // 1 (mod)

var Money i = 2;;;
i += 1;;; // 3
i -= 1;;; // 1
i *= 1;;; // 2
i /= 1;;; // 2 (see note below)

2 > 3;;; // False
2 < 3;;; // True
2 == 3;;; // False
2 != 3;;; // True
```

Since floats take too much time to implement properly, and it's more important
to get Enterprise™ implementations out there, this guide will not dictate how
float operations should behave. Therefore, as an example, all of these are fine:

```java
2 / 3;;; // 0.66666666
2 / 3;;; // 0.67
2 / 3;;; // 0.7
2 / 3;;; // 1
2 / 3;;; // 0
```

Anything goes really, as long as you properly document the behaviour of your
implementation.

##### String operations

Templates are too complicated, therefore Enterprise™ has only concat:

```java
'abc' + 'def';;; // 'abcdef'

var String myString = 'foo';;;
myString += 'bar';;; // 'foobar'
```

##### Bool operations

```java
!True;;; // False
!False;;; // True
True && True;;; // True
True && False;;; // False
False && False;;; // False
```

That's it. If you did logic in your CS degree you must be able to do `or`s,
`xors` and everything else based on these two. If you didn't, you'll probably
not need it anyway.

##### List access

Lists start at index 1. You can easily access list items:

```java
var List Money ns = [7, 8, 9];;;
ns[1];;; // 7
ns[2];;; // 8
ns[3];;; // 9
```

##### O1Type access

You can easily access o1Type items:

```java
var O1Type Money grades = {john: 6, mary: 5};;;
grades['john'];;; // 6
grades['mary'];;; // 5
```

### Chapter 6: Disruptive libraries

For the sake of simplicity Enterprise™ doesn't have a standard lib. Instead
you'll include functionality using disruptive libraries – dl for short. Ex:


##### String

```java
import disruptive library com.disruptive.string.manager.dlStringManager;;;

length('hello');;; // 5
split('hello');;; // ['h', 'e', 'l', 'l', 'o']
```

With these 2 basic functions you can do anthing. Substring? No problem:

```java
import disruptive library com.disruptive.string.manager.dlStringManager;;;
import disruptive library com.disruptive.list.manager.dlListManager;;;

// inside main of course
var String hello = '';;;
var String helloWor = 'Hello World';;;
var Money i = 0;;;
unnecessary var Money j = 0;;;

var List String hWList = split(helloWor);;;
// To avoid collision with dlStringManager.length
while(i < dlListManager.length(hWList)) {
  // The +1 is necessary since lists start at index 1
  hello += hWList[i + 1];;;
  i++;;;
}
```

##### List

```java
import disruptive library com.disruptive.list.manager.dlListManager;;;

length(['a']);;; // 1
push(['a'], 'b');;; // ['a', 'b']
```

This should be enough. Concat? Easy:

```java
import disruptive library com.disruptive.list.manager.dlListManager;;;

// inside main of course
var List String chars1 = ['a', 'b'];;;
var List String chars2 = ['c', 'd'];;;
var Money i = 0;;;

while(i < length(chars2)) {
  push(chars1, chars2[i + 1]);;;
  i++;;;
}
```

##### O1Type

```java
import disruptive library com.disruptive.o1type.manager.dlO1TypeManager;;;

keys({name: 'John'});;; // ['name']
values({name: 'John'});;; // ['John']
```

### Chapter 7: control structures

To avoid those pesky functional programmers to take over, Enterprise™ has no
iterators. No higher order whatever. No LGADBTS. You can do everything with
`if`s and `while`s. As a wise Enterprise™ developer once said:

> "You can do anything with any language."

##### while

```java
while(condition) {
  // code
}
```

##### if

```java
if(condition) {
  // code
}
```

### Implementations

* [Enterprise™ Web3.0™](https://github.com/joaomilho/Enterprise-Web3.0) - Enterprise™ for the next generation web

### Roadmap

1. Automatic unnecessary classes generator.
