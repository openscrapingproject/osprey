# Comparison of scraping extensions


ScrapeMate Beta- pretty good magic algorithm
CSS selectors work well

DataScraper - crap


Try XPath - interesting, good: 

//div[@class="css-698um9"]//a//h2

How to make a visual and interactive tool that hides opaque details like selector specification.

Goals:
- Use the most flexible specifiers possible (so that updates to site are not breaking)
- Capture the broadest swath of elements that are like what the user intended in their mind
- Allow the user to narrow down swath with additional qualifiers if that is not what they initially intended
- Allow for multiple positive groups, with negative groups inside each as well

E.g. on NYT, start by clicking on a title. 

Initially, would do `h2`. Then, user puts a higher-precedence/order specifier in place to restrict to the bottom section (`div.css-698um9`)

css-698um9

//h2/ancestor::a

//h2/ancestor::a/@href