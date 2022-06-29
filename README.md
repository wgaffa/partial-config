# Partial Config
This crate defines how to combine several config from different sources into a single config. The inspiration was taken from [Monoids for Configuration](https://ertes.eu/tutorial/config-monoids.html).

Some times you want to read configuration from different sources and not all sources may have all types of configuration available.

Here are some examples of sources some configuration might come from.
* Command Line Arguments
* Environment Variables
* Configuration Files
* Hard coded defaults in source code
* and more...

Note that it is up to the implementor to read data from the sources as this crate has no interfaces for that and is meant to perhaps have more freedom on how you read data.

