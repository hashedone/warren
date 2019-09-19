# What is Warren?
Warren is "programming in logic" library based on [Warren abstract machine](http://wambook.sourceforge.net/wambook.pdf). This is not another Prolog implementation, that means that doesn't include any runtime Prolog parsing or such things. It is purposed to have Prologish interpreter embeded in another software. The primary goal is to support type elision for [Typed](https://github.com/hashedone/typed), however because of it being very generic, I decided to extract it to completely external crate.

## Why not [Chalk](https://github.com/rust-lang/chalk)?
I find Chalk as greate piece of software, but I found three problems which makes it probably not best choice for my needs:
1. It is extremly Rust-focused - features which are implemented there are meant directly to be used by Rust compiler. Typed typesystem is meant to be far more generic, and I am aware of difficulties using it for my purposes
2. It is told that its api is unstable - quoting official README: "Other projects may of course re-use it too, if you have a need, but don't expect much stability in the interface yet.".
3. I didn't find any implementation of non-term constants/arithmetic, which makes implementing dependent-types at least tricky. I believe, this would be added in future (or maybe even soon) as long as it may be helpfull in implementing const-generics elision in Rust, but for not I don't think it is even in progress.

## Why not [Scryer Prolog](https://github.com/mthom/scryer-prolog)
Scryer prolog is another great programming in logic piece of art in Rust, but it is binary-focused. I could try to mess around it a little to extract the "engine part" but I am not willing to.

# Conclustion
I may be wrong with some of those points, but I didn't consider them too much - `Typed` is a toy project, so if I may learn more doing it, I find it as good opportunity. I don't focus on high performance (althrough I would also consider it while implementation), or being extermaly prolog-compliant.
