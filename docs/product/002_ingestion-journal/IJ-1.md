---
id: "IJ-1"
title: "Ingestion Logging"
document_type: "story"
author: @keinsell
---

# Ingestion Logging

Application introduces concept of "Ingestion Journal" which holds information about ingestions made by user (also known as "Subject") of application. Ingestion logging is important practice used in psychonautics, neurohacking and or even taking daily supplements to just remember what was ingested to avoid duplicate ingestions or do not skip daily ingestions. Ingestions are stored locally and eventually will be synchronized with external server (as encrypted information) however that's optional feature.

```
> neuronek ingestion create -s "Caffeine" -d "100mg" -r "oral"

// Ingestion "happy-pikachu" was created successfully.
```

Logging of ingestions should be plain and simple, allowing users to quickly log their ingestions and do not "feel effort" to do such, even for command-line application there should be "humanized" interface applied where users are able to use relative date or skip information when they do not know everything.

```
> neuronek ingestion create -s "Caffeine" -d "100mg" -at "30m ago" -r "oral"

// Ingestion "angry-charizzard" was created successfully.
// -----------------------------------------------------
// id: 13
// date: 30m ago (10 October 2013 12:30)
// substance: Caffeine
// administrated_by: Oral
// dosage: 100mg
// -----------------------------------------------------
```

Future iterations over this feature may include integration with analysis services which will utilize available amount of information to present analysis of ingestion to user just after one have inputed data into application.

  
```
> neuronek ingestion create -s "Caffeine" -d "100mg" -r "oral"

🧪 Caffeine
-------------------
Method: 🥤 Oral
Date: 📅 Now
Dosage: [###--] Common (100mg)
Duration: [#-----] 4 hours left (Onset for 30m) 
------------------
```

## Specification

- `log_ingestion` require information about `substance`, `dosage`, `administration_date` (defaults to `now`) and `route_of_administration` (defaults to `oral`)
- `log_ingestion` may contain `notes` which allow user to describe ingestion.
- `log_ingestion` must return created data after creation with `pretty`-format

## Test Cases

- [TC] Application must allow to log ingestion that happened now
- [TC] Application must allow to log ingestion that will happen in future
- [TC] Application must allow to log ingestion that happened in past
- [TC] Application must allow to use different dosage units
- [TC] Application must allow to use different dosage amounts
