/* eslint react/no-array-index-key: off */
import {Form, Formik} from "formik";
import React, {useEffect, useState} from "react";
import {EventObject} from "xstate";
import * as Yup from "yup";

import Button from "../common/button";
import Input from "../common/input";
import Textarea from "../common/text-area";
import {listMethodologies} from "../http";
import {FormProps, Methodology, MethodologySchema} from "../types";
import MethodologySelect from "./methodology-select";

type CreateInvestigationFormProps<CreateInvestigationFormValues> = FormProps<
  CreateInvestigationFormValues
>;

export interface CreateInvestigationFormValues {
  title: string;
  description: string;
  methodology: string;
}

export const defaultValues: CreateInvestigationFormValues = {
  title: "",
  description: "",
  methodology: "",
};

export const validationSchema = Yup.object({
  title: Yup.string().required("This field is required."),
  description: Yup.string(),
  methodology: Yup.string().required("This field is required."),
});

const CreateInvestigationForm = <
  TContext extends Record<string, unknown>,
  TStateSchema extends MethodologySchema,
  TEvent extends EventObject
>({
  initialValues = defaultValues,
  onCancel,
  onSubmit,
  workspace,
}: CreateInvestigationFormProps<CreateInvestigationFormValues>) => {
  const [methodologiesData, setMethodologiesData] = useState<
    Methodology<TContext, TStateSchema, TEvent>[]
  >([]);
  const formValues = {...defaultValues, ...initialValues};

  useEffect(() => {
    const fetchData = async () => {
      if (!workspace) return;
      const fetchedData: Methodology<
        TContext,
        TStateSchema,
        TEvent
      >[] = await listMethodologies(workspace.slug);
      setMethodologiesData(fetchedData);
    };
    fetchData();
  }, [workspace]);

  return (
    <Formik
      initialValues={formValues}
      validationSchema={validationSchema}
      onSubmit={onSubmit}
    >
      {({isValid, isSubmitting}) => {
        const disableSubmit = !isValid || isSubmitting;

        return (
          <Form>
            <Input label="Investigation Title" name="title" placeholder="" />

            <Textarea label="Description" name="description" placeholder="" />

            <MethodologySelect methodologies={methodologiesData} />

            <div className="flex justify-between ml-auto w-80 pv3 ">
              <Button
                type="reset"
                size="large"
                kind="secondary"
                onClick={onCancel}
              >
                Cancel
              </Button>

              <Button
                className="fr"
                type="submit"
                size="large"
                disabled={disableSubmit}
              >
                Create Investigation
              </Button>
            </div>
          </Form>
        );
      }}
    </Formik>
  );
};

export default CreateInvestigationForm;
