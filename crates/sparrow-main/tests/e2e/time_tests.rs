//! Basic e2e tests for the time operators.
//!
//! # Untested Time Functions
//!
//! The following functions do not have small unit tests in this file,
//! for the reasons listed:
//!
//! * `days()`: Because the CSV writer can't write `interval_days`. This is
//!   tested indirectly in other operations, such as `add_time_interval_days`.
//! * `months()`: Same situation as `days`. Tested indirectly by
//!   `add_time_interval_months`.
//! * `seconds()`: Same situation as `days`. Tested indirectly by
//!   `add_time_duration_s`.

use crate::fixtures::{
    boolean_data_fixture, i64_data_fixture, strings_data_fixture, timestamp_ns_data_fixture,
};
use crate::QueryFixture;

#[tokio::test]
async fn test_time_of_boolean() {
    insta::assert_snapshot!(QueryFixture::new("{ time_of: time_of(Booleans.a)}").run_to_csv(&boolean_data_fixture()).await.unwrap(), @r###"
    _time,_subsort,_key_hash,_key,time_of
    1996-12-20T00:39:57.000000000,9223372036854775808,3650215962958587783,A,1996-12-20T00:39:57.000000000
    1996-12-20T00:40:57.000000000,9223372036854775808,11753611437813598533,B,1996-12-20T00:40:57.000000000
    1996-12-20T00:41:57.000000000,9223372036854775808,11753611437813598533,B,1996-12-20T00:41:57.000000000
    1996-12-20T00:42:57.000000000,9223372036854775808,11753611437813598533,B,1996-12-20T00:42:57.000000000
    1996-12-20T00:43:57.000000000,9223372036854775808,11753611437813598533,B,1996-12-20T00:43:57.000000000
    1996-12-20T00:44:57.000000000,9223372036854775808,11753611437813598533,B,1996-12-20T00:44:57.000000000
    1996-12-20T00:45:57.000000000,9223372036854775808,11753611437813598533,B,1996-12-20T00:45:57.000000000
    "###);
}

#[tokio::test]
async fn test_time_of_i64() {
    insta::assert_snapshot!(QueryFixture::new("{ time_of: time_of(Numbers.m)}").run_to_csv(&i64_data_fixture()).await.unwrap(), @r###"
    _time,_subsort,_key_hash,_key,time_of
    1996-12-20T00:39:57.000000000,9223372036854775808,3650215962958587783,A,1996-12-20T00:39:57.000000000
    1996-12-20T00:39:58.000000000,9223372036854775808,11753611437813598533,B,1996-12-20T00:39:58.000000000
    1996-12-20T00:39:59.000000000,9223372036854775808,3650215962958587783,A,1996-12-20T00:39:59.000000000
    1996-12-20T00:40:00.000000000,9223372036854775808,3650215962958587783,A,1996-12-20T00:40:00.000000000
    1996-12-20T00:40:01.000000000,9223372036854775808,3650215962958587783,A,1996-12-20T00:40:01.000000000
    1996-12-20T00:40:02.000000000,9223372036854775808,3650215962958587783,A,1996-12-20T00:40:02.000000000
    "###);
}

#[tokio::test]
async fn test_time_of_timestamp() {
    insta::assert_snapshot!(QueryFixture::new("{ time_of: time_of(Times.time) }").run_to_csv(&timestamp_ns_data_fixture()).await.unwrap(), @r###"
    _time,_subsort,_key_hash,_key,time_of
    1994-12-20T00:39:57.000000000,9223372036854775808,3650215962958587783,A,1994-12-20T00:39:57.000000000
    1995-10-20T00:40:57.000000000,9223372036854775808,11753611437813598533,B,1995-10-20T00:40:57.000000000
    1996-08-20T00:41:57.000000000,9223372036854775808,11753611437813598533,B,1996-08-20T00:41:57.000000000
    1997-12-12T00:42:57.000000000,9223372036854775808,11753611437813598533,B,1997-12-12T00:42:57.000000000
    1998-12-13T00:43:57.000000000,9223372036854775808,11753611437813598533,B,1998-12-13T00:43:57.000000000
    2004-12-06T00:44:57.000000000,9223372036854775808,11753611437813598533,B,2004-12-06T00:44:57.000000000
    "###);
}

#[tokio::test]
async fn test_time_of_string() {
    insta::assert_snapshot!(QueryFixture::new("{ time_of: time_of(Strings.s)}").run_to_csv(&strings_data_fixture()).await.unwrap(), @r###"
    _time,_subsort,_key_hash,_key,time_of
    1996-12-20T00:39:57.000000000,9223372036854775808,3650215962958587783,A,1996-12-20T00:39:57.000000000
    1996-12-20T00:40:57.000000000,9223372036854775808,11753611437813598533,B,1996-12-20T00:40:57.000000000
    1996-12-20T00:41:57.000000000,9223372036854775808,11753611437813598533,B,1996-12-20T00:41:57.000000000
    1996-12-20T00:42:57.000000000,9223372036854775808,11753611437813598533,B,1996-12-20T00:42:57.000000000
    1996-12-20T00:43:57.000000000,9223372036854775808,11753611437813598533,B,1996-12-20T00:43:57.000000000
    1996-12-20T00:44:57.000000000,9223372036854775808,11753611437813598533,B,1996-12-20T00:44:57.000000000
    "###);
}

#[tokio::test]
async fn test_time_of_record() {
    insta::assert_snapshot!(QueryFixture::new("{ time_of: time_of(Strings)}").run_to_csv(&strings_data_fixture()).await.unwrap(), @r###"
    _time,_subsort,_key_hash,_key,time_of
    1996-12-20T00:39:57.000000000,9223372036854775808,3650215962958587783,A,1996-12-20T00:39:57.000000000
    1996-12-20T00:40:57.000000000,9223372036854775808,11753611437813598533,B,1996-12-20T00:40:57.000000000
    1996-12-20T00:41:57.000000000,9223372036854775808,11753611437813598533,B,1996-12-20T00:41:57.000000000
    1996-12-20T00:42:57.000000000,9223372036854775808,11753611437813598533,B,1996-12-20T00:42:57.000000000
    1996-12-20T00:43:57.000000000,9223372036854775808,11753611437813598533,B,1996-12-20T00:43:57.000000000
    1996-12-20T00:44:57.000000000,9223372036854775808,11753611437813598533,B,1996-12-20T00:44:57.000000000
    "###);
}

#[tokio::test]
async fn test_time_of_record_as_i64() {
    insta::assert_snapshot!(QueryFixture::new("{ time_of: time_of(Strings) as i64}").run_to_csv(&strings_data_fixture()).await.unwrap(), @r###"
    _time,_subsort,_key_hash,_key,time_of
    1996-12-20T00:39:57.000000000,9223372036854775808,3650215962958587783,A,851042397000000000
    1996-12-20T00:40:57.000000000,9223372036854775808,11753611437813598533,B,851042457000000000
    1996-12-20T00:41:57.000000000,9223372036854775808,11753611437813598533,B,851042517000000000
    1996-12-20T00:42:57.000000000,9223372036854775808,11753611437813598533,B,851042577000000000
    1996-12-20T00:43:57.000000000,9223372036854775808,11753611437813598533,B,851042637000000000
    1996-12-20T00:44:57.000000000,9223372036854775808,11753611437813598533,B,851042697000000000
    "###);
}

#[tokio::test]
async fn test_day_of_month() {
    insta::assert_snapshot!(QueryFixture::new("{ day_of_month: day_of_month(Times.time) }").run_to_csv(&timestamp_ns_data_fixture()).await.unwrap(), @r###"
    _time,_subsort,_key_hash,_key,day_of_month
    1994-12-20T00:39:57.000000000,9223372036854775808,3650215962958587783,A,20
    1995-10-20T00:40:57.000000000,9223372036854775808,11753611437813598533,B,20
    1996-08-20T00:41:57.000000000,9223372036854775808,11753611437813598533,B,20
    1997-12-12T00:42:57.000000000,9223372036854775808,11753611437813598533,B,12
    1998-12-13T00:43:57.000000000,9223372036854775808,11753611437813598533,B,13
    2004-12-06T00:44:57.000000000,9223372036854775808,11753611437813598533,B,6
    "###);
}

#[tokio::test]
async fn test_day_of_month0() {
    insta::assert_snapshot!(QueryFixture::new("{ day_of_month0: day_of_month0(Times.time) }").run_to_csv(&timestamp_ns_data_fixture()).await.unwrap(), @r###"
    _time,_subsort,_key_hash,_key,day_of_month0
    1994-12-20T00:39:57.000000000,9223372036854775808,3650215962958587783,A,19
    1995-10-20T00:40:57.000000000,9223372036854775808,11753611437813598533,B,19
    1996-08-20T00:41:57.000000000,9223372036854775808,11753611437813598533,B,19
    1997-12-12T00:42:57.000000000,9223372036854775808,11753611437813598533,B,11
    1998-12-13T00:43:57.000000000,9223372036854775808,11753611437813598533,B,12
    2004-12-06T00:44:57.000000000,9223372036854775808,11753611437813598533,B,5
    "###);
}

#[tokio::test]
async fn test_day_of_year() {
    insta::assert_snapshot!(QueryFixture::new("{ day_of_year: day_of_year(Times.time) }").run_to_csv(&timestamp_ns_data_fixture()).await.unwrap(), @r###"
    _time,_subsort,_key_hash,_key,day_of_year
    1994-12-20T00:39:57.000000000,9223372036854775808,3650215962958587783,A,354
    1995-10-20T00:40:57.000000000,9223372036854775808,11753611437813598533,B,293
    1996-08-20T00:41:57.000000000,9223372036854775808,11753611437813598533,B,233
    1997-12-12T00:42:57.000000000,9223372036854775808,11753611437813598533,B,346
    1998-12-13T00:43:57.000000000,9223372036854775808,11753611437813598533,B,347
    2004-12-06T00:44:57.000000000,9223372036854775808,11753611437813598533,B,341
    "###);
}

#[tokio::test]
async fn test_day_of_year0() {
    insta::assert_snapshot!(QueryFixture::new("{ day_of_year0: day_of_year0(Times.time) }").run_to_csv(&timestamp_ns_data_fixture()).await.unwrap(), @r###"
    _time,_subsort,_key_hash,_key,day_of_year0
    1994-12-20T00:39:57.000000000,9223372036854775808,3650215962958587783,A,353
    1995-10-20T00:40:57.000000000,9223372036854775808,11753611437813598533,B,292
    1996-08-20T00:41:57.000000000,9223372036854775808,11753611437813598533,B,232
    1997-12-12T00:42:57.000000000,9223372036854775808,11753611437813598533,B,345
    1998-12-13T00:43:57.000000000,9223372036854775808,11753611437813598533,B,346
    2004-12-06T00:44:57.000000000,9223372036854775808,11753611437813598533,B,340
    "###);
}

#[tokio::test]
async fn test_month_of_year() {
    insta::assert_snapshot!(QueryFixture::new("{ month_of_year: month_of_year(Times.time) }").run_to_csv(&timestamp_ns_data_fixture()).await.unwrap(), @r###"
    _time,_subsort,_key_hash,_key,month_of_year
    1994-12-20T00:39:57.000000000,9223372036854775808,3650215962958587783,A,12
    1995-10-20T00:40:57.000000000,9223372036854775808,11753611437813598533,B,10
    1996-08-20T00:41:57.000000000,9223372036854775808,11753611437813598533,B,8
    1997-12-12T00:42:57.000000000,9223372036854775808,11753611437813598533,B,12
    1998-12-13T00:43:57.000000000,9223372036854775808,11753611437813598533,B,12
    2004-12-06T00:44:57.000000000,9223372036854775808,11753611437813598533,B,12
    "###);
}

#[tokio::test]
async fn test_month_of_year0() {
    insta::assert_snapshot!(QueryFixture::new("{ month_of_year0: month_of_year0(Times.time) }").run_to_csv(&timestamp_ns_data_fixture()).await.unwrap(), @r###"
    _time,_subsort,_key_hash,_key,month_of_year0
    1994-12-20T00:39:57.000000000,9223372036854775808,3650215962958587783,A,11
    1995-10-20T00:40:57.000000000,9223372036854775808,11753611437813598533,B,9
    1996-08-20T00:41:57.000000000,9223372036854775808,11753611437813598533,B,7
    1997-12-12T00:42:57.000000000,9223372036854775808,11753611437813598533,B,11
    1998-12-13T00:43:57.000000000,9223372036854775808,11753611437813598533,B,11
    2004-12-06T00:44:57.000000000,9223372036854775808,11753611437813598533,B,11
    "###);
}

#[tokio::test]
async fn test_year() {
    insta::assert_snapshot!(QueryFixture::new("{ year: year(Times.time) }").run_to_csv(&timestamp_ns_data_fixture()).await.unwrap(), @r###"
    _time,_subsort,_key_hash,_key,year
    1994-12-20T00:39:57.000000000,9223372036854775808,3650215962958587783,A,1994
    1995-10-20T00:40:57.000000000,9223372036854775808,11753611437813598533,B,1995
    1996-08-20T00:41:57.000000000,9223372036854775808,11753611437813598533,B,1996
    1997-12-12T00:42:57.000000000,9223372036854775808,11753611437813598533,B,1997
    1998-12-13T00:43:57.000000000,9223372036854775808,11753611437813598533,B,1998
    2004-12-06T00:44:57.000000000,9223372036854775808,11753611437813598533,B,2004
    "###);
}

#[tokio::test]
async fn test_add_time_duration_s() {
    insta::assert_snapshot!(QueryFixture::new("{ add_time: Times.time | add_time(seconds(Times.n)) }").run_to_csv(&timestamp_ns_data_fixture()).await.unwrap(), @r###"
    _time,_subsort,_key_hash,_key,add_time
    1994-12-20T00:39:57.000000000,9223372036854775808,3650215962958587783,A,1994-12-20T00:39:59.000000000
    1995-10-20T00:40:57.000000000,9223372036854775808,11753611437813598533,B,1995-10-20T00:41:01.000000000
    1996-08-20T00:41:57.000000000,9223372036854775808,11753611437813598533,B,1996-08-20T00:42:02.000000000
    1997-12-12T00:42:57.000000000,9223372036854775808,11753611437813598533,B,
    1998-12-13T00:43:57.000000000,9223372036854775808,11753611437813598533,B,1998-12-13T00:44:05.000000000
    2004-12-06T00:44:57.000000000,9223372036854775808,11753611437813598533,B,2004-12-06T00:45:20.000000000
    "###);
}

#[tokio::test]
async fn test_add_time_duration_s_to_literal() {
    // This ensures that a string literal may be treated as a timestamp.
    insta::assert_snapshot!(QueryFixture::new("{ add_time: \"1994-12-20T00:39:59.000000000Z\" | add_time(seconds(Times.n)) }").run_to_csv(&timestamp_ns_data_fixture()).await.unwrap(), @r###"
    _time,_subsort,_key_hash,_key,add_time
    1994-12-20T00:39:57.000000000,9223372036854775808,3650215962958587783,A,1994-12-20T00:40:01.000000000
    1995-10-20T00:40:57.000000000,9223372036854775808,11753611437813598533,B,1994-12-20T00:40:03.000000000
    1996-08-20T00:41:57.000000000,9223372036854775808,11753611437813598533,B,1994-12-20T00:40:04.000000000
    1997-12-12T00:42:57.000000000,9223372036854775808,11753611437813598533,B,
    1998-12-13T00:43:57.000000000,9223372036854775808,11753611437813598533,B,1994-12-20T00:40:07.000000000
    2004-12-06T00:44:57.000000000,9223372036854775808,11753611437813598533,B,1994-12-20T00:40:22.000000000
    "###);
}

#[tokio::test]
async fn test_add_time_duration_s_literal() {
    insta::assert_snapshot!(QueryFixture::new("{ add_time: Times.time | add_time(seconds(10000)) }").run_to_csv(&timestamp_ns_data_fixture()).await.unwrap(), @r###"
    _time,_subsort,_key_hash,_key,add_time
    1994-12-20T00:39:57.000000000,9223372036854775808,3650215962958587783,A,1994-12-20T03:26:37.000000000
    1995-10-20T00:40:57.000000000,9223372036854775808,11753611437813598533,B,1995-10-20T03:27:37.000000000
    1996-08-20T00:41:57.000000000,9223372036854775808,11753611437813598533,B,1996-08-20T03:28:37.000000000
    1997-12-12T00:42:57.000000000,9223372036854775808,11753611437813598533,B,1997-12-12T03:29:37.000000000
    1998-12-13T00:43:57.000000000,9223372036854775808,11753611437813598533,B,1998-12-13T03:30:37.000000000
    2004-12-06T00:44:57.000000000,9223372036854775808,11753611437813598533,B,2004-12-06T03:31:37.000000000
    "###);
}

#[tokio::test]
async fn test_add_time_interval_months() {
    insta::assert_snapshot!(QueryFixture::new("{ add_time: Times.time | add_time(months(Times.n)) }").run_to_csv(&timestamp_ns_data_fixture()).await.unwrap(), @r###"
    _time,_subsort,_key_hash,_key,add_time
    1994-12-20T00:39:57.000000000,9223372036854775808,3650215962958587783,A,1995-02-20T00:39:57.000000000
    1995-10-20T00:40:57.000000000,9223372036854775808,11753611437813598533,B,1996-02-20T00:40:57.000000000
    1996-08-20T00:41:57.000000000,9223372036854775808,11753611437813598533,B,1997-01-20T00:41:57.000000000
    1997-12-12T00:42:57.000000000,9223372036854775808,11753611437813598533,B,
    1998-12-13T00:43:57.000000000,9223372036854775808,11753611437813598533,B,1999-08-13T00:43:57.000000000
    2004-12-06T00:44:57.000000000,9223372036854775808,11753611437813598533,B,2006-11-06T00:44:57.000000000
    "###);
}

#[tokio::test]
async fn test_add_time_interval_months_literal() {
    insta::assert_snapshot!(QueryFixture::new("{ add_time: Times.time | add_time(months(27)) }").run_to_csv(&timestamp_ns_data_fixture()).await.unwrap(), @r###"
    _time,_subsort,_key_hash,_key,add_time
    1994-12-20T00:39:57.000000000,9223372036854775808,3650215962958587783,A,1997-03-20T00:39:57.000000000
    1995-10-20T00:40:57.000000000,9223372036854775808,11753611437813598533,B,1998-01-20T00:40:57.000000000
    1996-08-20T00:41:57.000000000,9223372036854775808,11753611437813598533,B,1998-11-20T00:41:57.000000000
    1997-12-12T00:42:57.000000000,9223372036854775808,11753611437813598533,B,2000-03-12T00:42:57.000000000
    1998-12-13T00:43:57.000000000,9223372036854775808,11753611437813598533,B,2001-03-13T00:43:57.000000000
    2004-12-06T00:44:57.000000000,9223372036854775808,11753611437813598533,B,2007-03-06T00:44:57.000000000
    "###);
}

#[tokio::test]
async fn test_add_time_interval_months_literal_negative() {
    insta::assert_snapshot!(QueryFixture::new("{ add_time: Times.time | add_time(months(-1)) }").run_to_csv(&timestamp_ns_data_fixture()).await.unwrap(), @r###"
    _time,_subsort,_key_hash,_key,add_time
    1994-12-20T00:39:57.000000000,9223372036854775808,3650215962958587783,A,1994-11-20T00:39:57.000000000
    1995-10-20T00:40:57.000000000,9223372036854775808,11753611437813598533,B,1995-09-20T00:40:57.000000000
    1996-08-20T00:41:57.000000000,9223372036854775808,11753611437813598533,B,1996-07-20T00:41:57.000000000
    1997-12-12T00:42:57.000000000,9223372036854775808,11753611437813598533,B,1997-11-12T00:42:57.000000000
    1998-12-13T00:43:57.000000000,9223372036854775808,11753611437813598533,B,1998-11-13T00:43:57.000000000
    2004-12-06T00:44:57.000000000,9223372036854775808,11753611437813598533,B,2004-11-06T00:44:57.000000000
    "###);
}

#[tokio::test]
async fn test_add_time_interval_days() {
    insta::assert_snapshot!(QueryFixture::new("{ add_time: Times.time | add_time(days(Times.n)) }").run_to_csv(&timestamp_ns_data_fixture()).await.unwrap(), @r###"
    _time,_subsort,_key_hash,_key,add_time
    1994-12-20T00:39:57.000000000,9223372036854775808,3650215962958587783,A,1994-12-22T00:39:57.000000000
    1995-10-20T00:40:57.000000000,9223372036854775808,11753611437813598533,B,1995-10-24T00:40:57.000000000
    1996-08-20T00:41:57.000000000,9223372036854775808,11753611437813598533,B,1996-08-25T00:41:57.000000000
    1997-12-12T00:42:57.000000000,9223372036854775808,11753611437813598533,B,
    1998-12-13T00:43:57.000000000,9223372036854775808,11753611437813598533,B,1998-12-21T00:43:57.000000000
    2004-12-06T00:44:57.000000000,9223372036854775808,11753611437813598533,B,2004-12-29T00:44:57.000000000
    "###);
}

#[tokio::test]
async fn test_add_time_interval_days_literal() {
    insta::assert_snapshot!(QueryFixture::new("{ add_time: Times.time | add_time(days(372)) }").run_to_csv(&timestamp_ns_data_fixture()).await.unwrap(), @r###"
    _time,_subsort,_key_hash,_key,add_time
    1994-12-20T00:39:57.000000000,9223372036854775808,3650215962958587783,A,1995-12-27T00:39:57.000000000
    1995-10-20T00:40:57.000000000,9223372036854775808,11753611437813598533,B,1996-10-26T00:40:57.000000000
    1996-08-20T00:41:57.000000000,9223372036854775808,11753611437813598533,B,1997-08-27T00:41:57.000000000
    1997-12-12T00:42:57.000000000,9223372036854775808,11753611437813598533,B,1998-12-19T00:42:57.000000000
    1998-12-13T00:43:57.000000000,9223372036854775808,11753611437813598533,B,1999-12-20T00:43:57.000000000
    2004-12-06T00:44:57.000000000,9223372036854775808,11753611437813598533,B,2005-12-13T00:44:57.000000000
    "###);
}

#[tokio::test]
async fn test_seconds_between() {
    insta::assert_snapshot!(QueryFixture::new("let time = Times.time
                let other_time = Times.other_time
                let seconds_between = seconds_between(time, other_time) as i64
                in { time, other_time, seconds_between }").run_to_csv(&timestamp_ns_data_fixture()).await.unwrap(), @r###"
    _time,_subsort,_key_hash,_key,time,other_time,seconds_between
    1994-12-20T00:39:57.000000000,9223372036854775808,3650215962958587783,A,1994-12-19T16:39:57-08:00,2003-12-19T16:39:57-08:00,283996800
    1995-10-20T00:40:57.000000000,9223372036854775808,11753611437813598533,B,1995-10-19T16:40:57-08:00,1994-11-19T16:39:57-08:00,-28857660
    1996-08-20T00:41:57.000000000,9223372036854775808,11753611437813598533,B,1996-08-19T16:41:57-08:00,1998-12-19T16:39:57-08:00,73612680
    1997-12-12T00:42:57.000000000,9223372036854775808,11753611437813598533,B,1997-12-11T16:42:57-08:00,1992-12-19T16:39:57-08:00,-157075380
    1998-12-13T00:43:57.000000000,9223372036854775808,11753611437813598533,B,1998-12-12T16:43:57-08:00,,
    2004-12-06T00:44:57.000000000,9223372036854775808,11753611437813598533,B,2004-12-05T16:44:57-08:00,1994-12-19T16:39:57-08:00,-314409900
    "###);
}

#[tokio::test]
async fn test_days_between() {
    insta::assert_snapshot!(QueryFixture::new("let time = Times.time
                let other_time = Times.other_time
                let days_between = days_between(time, other_time) as i32
                in { time, other_time, days_between }").run_to_csv(&timestamp_ns_data_fixture()).await.unwrap(), @r###"
    _time,_subsort,_key_hash,_key,time,other_time,days_between
    1994-12-20T00:39:57.000000000,9223372036854775808,3650215962958587783,A,1994-12-19T16:39:57-08:00,2003-12-19T16:39:57-08:00,3287
    1995-10-20T00:40:57.000000000,9223372036854775808,11753611437813598533,B,1995-10-19T16:40:57-08:00,1994-11-19T16:39:57-08:00,-334
    1996-08-20T00:41:57.000000000,9223372036854775808,11753611437813598533,B,1996-08-19T16:41:57-08:00,1998-12-19T16:39:57-08:00,851
    1997-12-12T00:42:57.000000000,9223372036854775808,11753611437813598533,B,1997-12-11T16:42:57-08:00,1992-12-19T16:39:57-08:00,-1818
    1998-12-13T00:43:57.000000000,9223372036854775808,11753611437813598533,B,1998-12-12T16:43:57-08:00,,
    2004-12-06T00:44:57.000000000,9223372036854775808,11753611437813598533,B,2004-12-05T16:44:57-08:00,1994-12-19T16:39:57-08:00,-3639
    "###);

    // Tests that interval_days can cast to other types
    insta::assert_snapshot!(QueryFixture::new("let time = Times.time
                let other_time = Times.other_time
                let days_between = days_between(time, other_time) as i64
                in { time, other_time, days_between }").run_to_csv(&timestamp_ns_data_fixture()).await.unwrap(), @r###"
    _time,_subsort,_key_hash,_key,time,other_time,days_between
    1994-12-20T00:39:57.000000000,9223372036854775808,3650215962958587783,A,1994-12-19T16:39:57-08:00,2003-12-19T16:39:57-08:00,3287
    1995-10-20T00:40:57.000000000,9223372036854775808,11753611437813598533,B,1995-10-19T16:40:57-08:00,1994-11-19T16:39:57-08:00,-334
    1996-08-20T00:41:57.000000000,9223372036854775808,11753611437813598533,B,1996-08-19T16:41:57-08:00,1998-12-19T16:39:57-08:00,851
    1997-12-12T00:42:57.000000000,9223372036854775808,11753611437813598533,B,1997-12-11T16:42:57-08:00,1992-12-19T16:39:57-08:00,-1818
    1998-12-13T00:43:57.000000000,9223372036854775808,11753611437813598533,B,1998-12-12T16:43:57-08:00,,
    2004-12-06T00:44:57.000000000,9223372036854775808,11753611437813598533,B,2004-12-05T16:44:57-08:00,1994-12-19T16:39:57-08:00,-3639
    "###);

    insta::assert_snapshot!(QueryFixture::new("let time = Times.time
                let other_time = Times.other_time
                let days_between = days_between(time, other_time) as f32
                in { time, other_time, days_between }").run_to_csv(&timestamp_ns_data_fixture()).await.unwrap(), @r###"
    _time,_subsort,_key_hash,_key,time,other_time,days_between
    1994-12-20T00:39:57.000000000,9223372036854775808,3650215962958587783,A,1994-12-19T16:39:57-08:00,2003-12-19T16:39:57-08:00,3287.0
    1995-10-20T00:40:57.000000000,9223372036854775808,11753611437813598533,B,1995-10-19T16:40:57-08:00,1994-11-19T16:39:57-08:00,-334.0
    1996-08-20T00:41:57.000000000,9223372036854775808,11753611437813598533,B,1996-08-19T16:41:57-08:00,1998-12-19T16:39:57-08:00,851.0
    1997-12-12T00:42:57.000000000,9223372036854775808,11753611437813598533,B,1997-12-11T16:42:57-08:00,1992-12-19T16:39:57-08:00,-1818.0
    1998-12-13T00:43:57.000000000,9223372036854775808,11753611437813598533,B,1998-12-12T16:43:57-08:00,,
    2004-12-06T00:44:57.000000000,9223372036854775808,11753611437813598533,B,2004-12-05T16:44:57-08:00,1994-12-19T16:39:57-08:00,-3639.0
    "###);
}

#[tokio::test]
async fn test_months_between() {
    insta::assert_snapshot!(QueryFixture::new("let time = Times.time
                let other_time = Times.other_time
                let months_between = months_between(time, other_time) as i32
                in { time, other_time, months_between }").run_to_csv(&timestamp_ns_data_fixture()).await.unwrap(), @r###"
    _time,_subsort,_key_hash,_key,time,other_time,months_between
    1994-12-20T00:39:57.000000000,9223372036854775808,3650215962958587783,A,1994-12-19T16:39:57-08:00,2003-12-19T16:39:57-08:00,108
    1995-10-20T00:40:57.000000000,9223372036854775808,11753611437813598533,B,1995-10-19T16:40:57-08:00,1994-11-19T16:39:57-08:00,-11
    1996-08-20T00:41:57.000000000,9223372036854775808,11753611437813598533,B,1996-08-19T16:41:57-08:00,1998-12-19T16:39:57-08:00,28
    1997-12-12T00:42:57.000000000,9223372036854775808,11753611437813598533,B,1997-12-11T16:42:57-08:00,1992-12-19T16:39:57-08:00,-60
    1998-12-13T00:43:57.000000000,9223372036854775808,11753611437813598533,B,1998-12-12T16:43:57-08:00,,
    2004-12-06T00:44:57.000000000,9223372036854775808,11753611437813598533,B,2004-12-05T16:44:57-08:00,1994-12-19T16:39:57-08:00,-120
    "###);
}

#[tokio::test]
async fn test_seconds_between_literal() {
    insta::assert_snapshot!(QueryFixture::new("
        let time = Times.time
        let seconds_between = seconds_between(time, \"1994-12-20T00:41:57.000000000-08:00\") as i64
        in { time, seconds_between }").run_to_csv(&timestamp_ns_data_fixture()).await.unwrap(), @r###"
    _time,_subsort,_key_hash,_key,time,seconds_between
    1994-12-20T00:39:57.000000000,9223372036854775808,3650215962958587783,A,1994-12-19T16:39:57-08:00,28920
    1995-10-20T00:40:57.000000000,9223372036854775808,11753611437813598533,B,1995-10-19T16:40:57-08:00,-26236740
    1996-08-20T00:41:57.000000000,9223372036854775808,11753611437813598533,B,1996-08-19T16:41:57-08:00,-52588800
    1997-12-12T00:42:57.000000000,9223372036854775808,11753611437813598533,B,1997-12-11T16:42:57-08:00,-93974460
    1998-12-13T00:43:57.000000000,9223372036854775808,11753611437813598533,B,1998-12-12T16:43:57-08:00,-125596920
    2004-12-06T00:44:57.000000000,9223372036854775808,11753611437813598533,B,2004-12-05T16:44:57-08:00,-314380980
    "###);
}

#[tokio::test]
async fn test_days_between_literal() {
    insta::assert_snapshot!(QueryFixture::new("
        let time = Times.time
        let days_between = days_between(time, \"1994-12-20T00:41:57.000000000-08:00\") as i32
        in { time, days_between }").run_to_csv(&timestamp_ns_data_fixture()).await.unwrap(), @r###"
    _time,_subsort,_key_hash,_key,time,days_between
    1994-12-20T00:39:57.000000000,9223372036854775808,3650215962958587783,A,1994-12-19T16:39:57-08:00,0
    1995-10-20T00:40:57.000000000,9223372036854775808,11753611437813598533,B,1995-10-19T16:40:57-08:00,-303
    1996-08-20T00:41:57.000000000,9223372036854775808,11753611437813598533,B,1996-08-19T16:41:57-08:00,-608
    1997-12-12T00:42:57.000000000,9223372036854775808,11753611437813598533,B,1997-12-11T16:42:57-08:00,-1087
    1998-12-13T00:43:57.000000000,9223372036854775808,11753611437813598533,B,1998-12-12T16:43:57-08:00,-1453
    2004-12-06T00:44:57.000000000,9223372036854775808,11753611437813598533,B,2004-12-05T16:44:57-08:00,-3638
    "###);
}

#[tokio::test]
async fn test_months_between_literal() {
    insta::assert_snapshot!(QueryFixture::new("
        let time = Times.time
        let months_between = months_between(time, \"1994-12-20T00:41:57.000000000-08:00\") as i32
        in { time, months_between }").run_to_csv(&timestamp_ns_data_fixture()).await.unwrap(), @r###"
    _time,_subsort,_key_hash,_key,time,months_between
    1994-12-20T00:39:57.000000000,9223372036854775808,3650215962958587783,A,1994-12-19T16:39:57-08:00,0
    1995-10-20T00:40:57.000000000,9223372036854775808,11753611437813598533,B,1995-10-19T16:40:57-08:00,-10
    1996-08-20T00:41:57.000000000,9223372036854775808,11753611437813598533,B,1996-08-19T16:41:57-08:00,-20
    1997-12-12T00:42:57.000000000,9223372036854775808,11753611437813598533,B,1997-12-11T16:42:57-08:00,-36
    1998-12-13T00:43:57.000000000,9223372036854775808,11753611437813598533,B,1998-12-12T16:43:57-08:00,-48
    2004-12-06T00:44:57.000000000,9223372036854775808,11753611437813598533,B,2004-12-05T16:44:57-08:00,-120
    "###);
}

#[allow(unused_attributes)]
#[ignore = "lag(0, e) not currently supported"]
#[tokio::test]
async fn test_lag_0_i64() {
    // `lag(e, 0)` should be equivalent to `e` (if it doesn't produce continuous
    // values) or `last(e)` if it produces continuous values. Either way, it isn't
    // currently supported.
    //
    // It may also be desirable to convert `last(e)` to `lag(e, 0)` and support it
    // if it allows sharing more of the buffer.

    insta::assert_snapshot!(QueryFixture::new("{ m: Numbers.m, lag_zero: lag(Numbers.m) }").run_to_csv(&i64_data_fixture()).await.unwrap(), @r###"
    _time,_subsort,_key_hash,m,lag_zero
    1996-12-20T00:39:57.000000000,0,16072519723445549088,5,5
    1996-12-20T00:39:58.000000000,0,18113259709342437355,24,24
    1996-12-20T00:39:59.000000000,0,16072519723445549088,17,17
    1996-12-20T00:40:00.000000000,0,16072519723445549088,,17
    1996-12-20T00:40:01.000000000,0,16072519723445549088,12,12
    1996-12-20T00:40:02.000000000,0,16072519723445549088,,12
    "###)
}

#[tokio::test]
async fn test_lag_1_i64() {
    insta::assert_snapshot!(QueryFixture::new("{ m: Numbers.m, lag_one: lag(1, Numbers.m) }").run_to_csv(&i64_data_fixture()).await.unwrap(), @r###"
    _time,_subsort,_key_hash,_key,m,lag_one
    1996-12-20T00:39:57.000000000,9223372036854775808,3650215962958587783,A,5,
    1996-12-20T00:39:58.000000000,9223372036854775808,11753611437813598533,B,24,
    1996-12-20T00:39:59.000000000,9223372036854775808,3650215962958587783,A,17,5
    1996-12-20T00:40:00.000000000,9223372036854775808,3650215962958587783,A,,17
    1996-12-20T00:40:01.000000000,9223372036854775808,3650215962958587783,A,12,17
    1996-12-20T00:40:02.000000000,9223372036854775808,3650215962958587783,A,,12
    "###)
}

#[tokio::test]
async fn test_lag_2_i64() {
    insta::assert_snapshot!(QueryFixture::new("{ m: Numbers.m, lag_two: Numbers.m | lag(2) }").run_to_csv(&i64_data_fixture()).await.unwrap(), @r###"
    _time,_subsort,_key_hash,_key,m,lag_two
    1996-12-20T00:39:57.000000000,9223372036854775808,3650215962958587783,A,5,
    1996-12-20T00:39:58.000000000,9223372036854775808,11753611437813598533,B,24,
    1996-12-20T00:39:59.000000000,9223372036854775808,3650215962958587783,A,17,
    1996-12-20T00:40:00.000000000,9223372036854775808,3650215962958587783,A,,5
    1996-12-20T00:40:01.000000000,9223372036854775808,3650215962958587783,A,12,5
    1996-12-20T00:40:02.000000000,9223372036854775808,3650215962958587783,A,,17
    "###)
}

#[tokio::test]
async fn test_mean_time_between() {
    insta::assert_snapshot!(QueryFixture::new("
        let curr = time_of(Times)
        let prev = curr | lag(1)
        let elapsed = seconds_between(prev, curr) as i64
        in { prev, curr, elapsed, mean_elapsed: mean(elapsed) }").run_to_csv(&timestamp_ns_data_fixture()).await.unwrap(),
        @r###"
    _time,_subsort,_key_hash,_key,prev,curr,elapsed,mean_elapsed
    1994-12-20T00:39:57.000000000,9223372036854775808,3650215962958587783,A,,1994-12-20T00:39:57.000000000,,
    1995-10-20T00:40:57.000000000,9223372036854775808,11753611437813598533,B,,1995-10-20T00:40:57.000000000,,
    1996-08-20T00:41:57.000000000,9223372036854775808,11753611437813598533,B,1995-10-20T00:40:57.000000000,1996-08-20T00:41:57.000000000,26352060,26352060.0
    1997-12-12T00:42:57.000000000,9223372036854775808,11753611437813598533,B,1996-08-20T00:41:57.000000000,1997-12-12T00:42:57.000000000,41385660,33868860.0
    1998-12-13T00:43:57.000000000,9223372036854775808,11753611437813598533,B,1997-12-12T00:42:57.000000000,1998-12-13T00:43:57.000000000,31622460,33120060.0
    2004-12-06T00:44:57.000000000,9223372036854775808,11753611437813598533,B,1998-12-13T00:43:57.000000000,2004-12-06T00:44:57.000000000,188784060,72036060.0
    "###)
}