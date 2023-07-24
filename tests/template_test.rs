use carbone_sdk_rs::errors::CarboneError;
use carbone_sdk_rs::template::*;

#[cfg(test)]
mod tests {

    use super::*;
    use anyhow::Result;
    use carbone_sdk_rs::template::TemplateId;

    #[test]
    fn test_template_file() -> Result<(), CarboneError> {
        let template_file_path = "tests/data/template.test.odt";
        let template_file = TemplateFile::new(template_file_path.to_string())?;

        assert_eq!(template_file.path_as_str(), template_file_path);

        Ok(())
    }

    #[test]
    fn test_template_file_directory_given() -> Result<(), CarboneError> {
        let template_file_path = "tests/data/";
        let result = TemplateFile::new(template_file_path.to_string());

        let expected_error = CarboneError::IsADirectory(template_file_path.to_string());

        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), expected_error.to_string());

        Ok(())
    }

    #[test]
    fn test_template_file_not_exists_given() -> Result<(), CarboneError> {
        let template_file_path = "tests/data/unknown_template.test.docx";
        let result = TemplateFile::new(template_file_path.to_string());

        let expected_error = CarboneError::TemplateFileNotFound(template_file_path.to_string());

        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), expected_error.to_string());

        Ok(())
    }

    #[test]
    fn test_generate_template_id_odt_1() -> Result<(), CarboneError> {
        let template_file_path = "tests/data/template.test.odt".to_string();
        let template_file = TemplateFile::new(template_file_path.to_string())?;
        let template_id = template_file.generate_id("")?;

        let expected_template_id = TemplateId::new(
            "0545253258577a632a99065f0572720225f5165cc43db9515e9cef0e17b40114".to_string(),
        )?;

        assert_eq!(expected_template_id, template_id);

        Ok(())
    }

    #[test]
    fn test_generate_template_id_odt_2_payload_1() -> Result<(), CarboneError> {
        let file_name = "tests/data/template.test.odt".to_string();
        let template_file = TemplateFile::new(file_name.to_string())?;
        let template_id = template_file.generate_id("ThisIsAPayload")?;

        let expected_template_id = TemplateId::new(
            "7de8d1d8676abb32291ea5119cb1f78fe37fdfdc75332fcdae28f1e30d064ac0".to_string(),
        )?;

        assert_eq!(expected_template_id, template_id);

        Ok(())
    }

    #[test]
    fn test_generate_template_id_odt_3_payload_2() -> Result<(), CarboneError> {
        let file_name = "tests/data/template.test.odt".to_string();
        let template_file = TemplateFile::new(file_name.to_string())?;
        let template_id = template_file.generate_id("8B5PmafbjdRqHuksjHNw83mvPiGj7WTE")?;

        let expected_template_id = TemplateId::new(
            "a62eb407a5d5765ddf974636de8ab47bda7915cebd61197d7a2bb42ae70ffcd6".to_string(),
        )?;

        assert_eq!(expected_template_id, template_id);

        Ok(())
    }

    #[test]
    fn test_generate_template_id_html_1() -> Result<(), CarboneError> {
        let template_file = TemplateFile::new("tests/data/template.test.html".to_string())?;
        let template_id = template_file.generate_id("")?;

        let expected_template_id = TemplateId::new(
            "75256dd5c260cdf039ae807d3a007e78791e2d8963ea1aa6aff87ba03074df7f".to_string(),
        )?;

        assert_eq!(expected_template_id, template_id);

        Ok(())
    }

    #[test]
    fn test_generate_template_id_html_2_payload_1() -> Result<(), CarboneError> {
        let template_file = TemplateFile::new("tests/data/template.test.html".to_string())?;
        let payload = "This is a long payload with different characters 1 *5 &*9 %$ 3%&@9 @(( 3992288282 29299 9299929";
        let template_id = template_file.generate_id(payload)?;

        let expected_template_id = TemplateId::new(
            "70799b421cc9cf75d9112273a8e054c141d484eb8d5988bd006fac83e3990707".to_string(),
        )?;

        assert_eq!(expected_template_id, template_id);

        Ok(())
    }

    #[test]
    fn test_template_id_as_ref() -> Result<(), CarboneError> {
        let id_value = "1";
        let id = TemplateId::new(id_value.to_string())?;

        assert_eq!(id.as_ref(), id_value);

        Ok(())
    }
}
