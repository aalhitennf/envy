Envy is very small and simple dotenv file library. It supports debug, release, test and custom profiles. It allows you to have different env variables for debug, release and test runs.

**Example usage**

Cargo.toml  

    [dependencies]
    envy = { git = "https://github.com/aalhitennf/envy" }

By default envy tries to read from following files in your project folder.  


    .env.debug
    .env.test
    .env


You can also create freely named custom file.

Define env file content:  

    # You can comment lines
    DB_PORT=1234
    DB_URI=http://localhost
    DB_PASSWORD=SecretPassword # You can also comment at end of lines


In code:

    use envy::Envy;

    // Use current() if you want to envy detect what profile is in use
    let envy = Envy::detect().unwrap();

    // debug() for values from .env.debug
    let envy = Envy::debug().unwrap();

    // release() for values from .env
    let envy = Envy::release().unwrap();

    // test() for values from .env.test
    let envy = Envy::test().unwrap();

    // try_from(path) for values from custom file
    let envy = Envy::try_from("mycustomfilepath").unwrap();
 
    // Get value. If value wasn't defined in env file, get return empty string
    // to avoid crashing at least
    let port = envy.get("PORT");



