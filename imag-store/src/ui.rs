use clap::{Arg, App, ArgGroup, SubCommand};

pub fn build_ui<'a>(app: App<'a, 'a>) -> App<'a, 'a> {
    app.subcommand(SubCommand::with_name("create")
                   .about("Create an entry from the store")
                   .version("0.1")
                   .arg(Arg::with_name("path")
                        .long("path")
                        .short("p")
                        .takes_value(true)
                        .required(false)
                        .help("Create at this store path")
                        .value_name("PATH"))
                   .arg(Arg::with_name("id")
                        .long("id")
                        .short("i")
                        .takes_value(true)
                        .required(false)
                        .help("Same as --path, for consistency")
                        .value_name("PATH"))
                   .arg(Arg::with_name("from-raw")
                        .long("from-raw")
                        .takes_value(true)
                        .help("Create a new entry by reading this file ('-' for stdin)")
                        .value_name("FILE"))

                   .group(ArgGroup::with_name("create-destination-group")
                          .args(&["path", "id"])
                          .required(true))

                   .subcommand(SubCommand::with_name("entry")
                               .about("Create an entry via commandline")
                               .version("0.1")
                               .arg(Arg::with_name("content")
                                    .long("content")
                                    .short("c")
                                    .takes_value(true)
                                    .help("Content for the Entry from commandline")
                                    .value_name("CONTENT"))
                               .arg(Arg::with_name("content-from")
                                    .long("content-from")
                                    .short("f")
                                    .takes_value(true)
                                    .help("Content for the Entry from this file ('-' for stdin)")
                                    .value_name("CONTENT"))

                               .group(ArgGroup::with_name("create-content-group")
                                      .args(&["content", "content-from"])
                                      .required(false))

                               .arg(Arg::with_name("header")
                                    .long("header")
                                    .short("h")
                                    .takes_value(true)
                                    .multiple(true)
                                    .help("Set a header field. Specify as 'header.field.value=value', multiple allowed")
                                    .value_name("header.field.value=value"))
                               )
                   )

       .subcommand(SubCommand::with_name("retrieve")
                   .about("Retrieve an entry from the store (implicitely creates the entry)")
                   .version("0.1")
                   .arg(Arg::with_name("id")
                        .long("id")
                        .short("i")
                        .takes_value(true)
                        .required(true)
                        .help("Retreive by Store Path, where root (/) is the store itself"))
                   .arg(Arg::with_name("content")
                        .long("content")
                        .short("c")
                        .help("Print content"))
                   .arg(Arg::with_name("header")
                        .long("header")
                        .short("h")
                        .help("Print header"))
                   .arg(Arg::with_name("header-json")
                        .long("header-json")
                        .short("j")
                        .help("Print header as json"))
                   .arg(Arg::with_name("raw")
                        .long("raw")
                        .short("r")
                        .help("Print Entries as they are in the store"))

                   .subcommand(SubCommand::with_name("filter-header")
                               .about("Retrieve Entries by filtering")
                               .version("0.1")
                               .arg(Arg::with_name("header-field-where")
                                    .long("where")
                                    .short("w")
                                    .takes_value(true)
                                    .help("Filter with 'header.field=foo' where the header field 'header.field' equals 'foo'")
                               )
                               .arg(Arg::with_name("header-field-grep")
                                    .long("grep")
                                    .short("g")
                                    .takes_value(true)
                                    .help("Filter with 'header.field=[a-zA-Z0-9]*' where the header field 'header.field' matches '[a-zA-Z0-9]*'"))
                               )
                   )

       .subcommand(SubCommand::with_name("get")
                   .about("Get an entry from the store (fails if non-existent)")
                   .version("0.1")
                   .arg(Arg::with_name("id")
                        .long("id")
                        .short("i")
                        .takes_value(true)
                        .required(true)
                        .help("Retrieve by Store Path, where root (/) is the store itself")
                        .value_name("PATH"))
                   .arg(Arg::with_name("content")
                        .long("content")
                        .short("c")
                        .help("Print content"))
                   .arg(Arg::with_name("header")
                        .long("header")
                        .short("h")
                        .help("Print header"))
                   .arg(Arg::with_name("header-json")
                        .long("header-json")
                        .short("j")
                        .help("Print header as json"))
                   .arg(Arg::with_name("raw")
                        .long("raw")
                        .short("r")
                        .help("Print Entries as they are in the store"))

                   .subcommand(SubCommand::with_name("filter-header")
                               .about("Retrieve Entries by filtering")
                               .version("0.1")
                               .arg(Arg::with_name("header-field-where")
                                    .long("where")
                                    .short("w")
                                    .takes_value(true)
                                    .help("Filter with 'header.field=foo' where the header field 'header.field' equals 'foo'")
                                    .value_name("header.field=foo")
                               )
                               .arg(Arg::with_name("header-field-grep")
                                    .long("grep")
                                    .short("g")
                                    .takes_value(true)
                                    .help("Filter with 'header.field=[a-zA-Z0-9]*' where the header field 'header.field' matches '[a-zA-Z0-9]*'"))
                               )
                   )

       .subcommand(SubCommand::with_name("update")
                   .about("Get an entry from the store")
                   .version("0.1")
                   .arg(Arg::with_name("id")
                        .long("id")
                        .short("i")
                        .takes_value(true)
                        .required(true)
                        .help("Update Store Entry with this path. Root (/) is the store itself")
                        .value_name("PATH"))
                   .arg(Arg::with_name("content")
                        .long("content")
                        .short("c")
                        .takes_value(true)
                        .help("Take the content for the new Entry from this file ('-' for stdin)")
                        .value_name("CONTENT"))
                   .arg(Arg::with_name("header")
                        .long("header")
                        .short("h")
                        .takes_value(true)
                        .multiple(true)
                        .help("Set a header field. Specify as 'header.field.value=value', multiple allowed"))
                   )

       .subcommand(SubCommand::with_name("delete")
                   .about("Delete an entry from the store")
                   .version("0.1")
                   .arg(Arg::with_name("id")
                        .long("id")
                        .short("i")
                        .takes_value(true)
                        .required(true)
                        .help("Remove Store Entry with this path. Root (/) is the store itself")
                        .value_name("PATH"))
                   )
}
