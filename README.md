# 🚧 WIP AND NOT WORKING: Backblaze B2 Kubernetes Operator 🏗️

This operator allows you to provision B2 Buckets and Keys from inside your K8s cluster.

## Idea

This operator was conceived while working at Scandio GmbH on a backup concept for K8s clusters using K8up. The value proposition is to automatically provision applications specific `Keys` and `Buckets` without having to handle these keys manually. key rotation is made easier as well since you can just delete and redeploy a `Key` and the operator will recreate it.

## Workflow

1. Create a Secret containing a B2 Access Key. This will be used to manage Buckets and Keys. It will have to have permissions to do so.
2. Create an `Account` custom resource referencing the secret
3. Create a `Bucket` custom resource to either create or adopt an existing Bucket. Deleting a `Bucket` CR won't delete the Bucket on B2. 
4. Create a `Key` custom resource to create a Key. Enter the specs of the target secret where the credentials will be stored for use by applications

## Features

- [x] Set up multiple "source" Accounts
- [ ] Create/Adopt `Buckets`
- [ ] Create `Keys`
- [ ] Generate Secrets from `Keys`
