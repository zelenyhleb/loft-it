/*******************************************************************************
 * Copyright (c) ArSysOp 2018-2022
 *
 * RGM Sources are publicly available only for
 * informational, review, analysis and consulting purposes.
 *
 * Definitions, terms and conditions for using RGM Sources are stated by ArSysOp Source License version 1.0.
 * See http://arsysop.ru/licenses/rgm/ArSysOpSourceLicense-1.0.txt
 *
 * RGM Sources are provided on "as is" basis.
 * ArSysOp is not responsible for any damages, losses, legal prosecution
 * or other consequences of any sort that using RGM Sources can cause to you
 * (as an individual or Legal Entity), even if aware of such consequences.
 *
*******************************************************************************/
use inflector::Inflector;
use serde::Deserialize;
use std::{
    fs::File,
    io::Write,
    path::{Path, PathBuf},
};

#[derive(Deserialize)]
pub struct TestConfiguration {
    pub directory: String,
    pub package: String,
    pub url: String,
}
pub struct Section {
    pub id: String,
    pub content: Vec<Section>,
}

pub fn create_test(section: Section, template: &TestConfiguration) {
    if String::from("").eq(&section.id) {
        return;
    }
    let content = content(&section, template);
    let file_name = class_name(&section);
    write_file(template, file_name, content);
}

pub fn create_base(template: &TestConfiguration) {
    let content = base_test(template);
    let name = String::from("CxxIntegrityTest");
    write_file(template, name, content);
}

fn write_file(template: &TestConfiguration, name: String, content: String) {
    let mut file = File::create(file_path(name, template)).unwrap();
    file.write_all(content.as_bytes()).unwrap();
}

fn file_path(name: String, template: &TestConfiguration) -> PathBuf {
    let dir = &template.directory;
    Path::new(dir).join(name + ".java".trim_end())
}

fn class_name(section: &Section) -> String {
    let mut class_name = section.id.replace(".", "_").to_class_case();
    class_name.push_str("IntegrityTest");
    return class_name;
}

fn method_name(section: &Section) -> String {
    section.id.replace(".", "_").to_camel_case()
}

fn single_test(section: &Section) -> String {
    let contained = section.content.len();
    if contained > 0 {
        format!(
            "
    @Test
    public void {}Contents() {{
        assertEquals({}, sections().stream() //
                .filter(section -> section.getId().equals(\"{}\")) //$NON-NLS-1$
                .flatMap(section -> section.getContents().stream()) //
                .filter(Section.class::isInstance) //
                .count());
    }}\n",
            method_name(section),
            contained,
            section.id
        )
    } else {
        String::from("")
    }
}

fn base_test(template: &TestConfiguration) -> String {
    format!("/*******************************************************************************
* Copyright (c) ArSysOp 2018-2022
* 
* RGM Sources are publicly available only for 
* informational, review, analysis and consulting purposes.
* 
* Definitions, terms and conditions for using RGM Sources are stated by ArSysOp Source License version 1.0.
* See http://arsysop.ru/licenses/rgm/ArSysOpSourceLicense-1.0.txt
* 
* RGM Sources are provided on \"as is\" basis. 
* ArSysOp is not responsible for any damages, losses, legal prosecution 
* or other consequences of any sort that using RGM Sources can cause to you 
* (as an individual or Legal Entity), even if aware of such consequences.
* 
*******************************************************************************/
package {};

import static org.junit.Assert.fail;

import java.util.Collection;
import java.util.HashSet;
import java.util.Set;

import org.eclipse.core.runtime.CoreException;
import org.eclipse.core.runtime.NullProgressMonitor;

import ru.arsysop.loft.rgm.cxxdraft.ResolutionContext;
import ru.arsysop.loft.rgm.cxxdraft.base.PublishedHtml;
import ru.arsysop.loft.rgm.cxxdraft.base.SimpleResolutionContext;
import ru.arsysop.loft.rgm.internal.cxxdraft.TocStructure;
import ru.arsysop.loft.rgm.spec.model.api.Document;
import ru.arsysop.loft.rgm.spec.model.api.Section;
import ru.arsysop.loft.rgm.spec.model.meta.SpecFactory;

@SuppressWarnings(\"restriction\")
public abstract class CxxIntegrityTest {{

	private static final HashSet<Section> sections = new HashSet<>();

	protected Set<Section> sections() {{
		if (sections.size() == 0) {{
			try {{
				performParsing();
			}} catch (CoreException e) {{
				fail(\"Can't download specification\"); //$NON-NLS-1$
			}}
		}}
		return sections;
	}}

	private void performParsing() throws CoreException {{
		Document document = SpecFactory.eINSTANCE.createDocument();
		String URL = \"{}\"; //$NON-NLS-1$
		ResolutionContext context = new SimpleResolutionContext(URL, document);
		document.setToc(SpecFactory.eINSTANCE.createToc());
		new PublishedHtml(//
				context.location(), //
				new TocStructure(document.getToc(), context)//
		).run(new NullProgressMonitor());
		document.getSections().stream() //
				.map(CxxIntegrityTest::expand) //
				.flatMap(Collection::stream) //
				.forEach(sections::add);
	}}

	private static Collection<Section> expand(Section section) {{
		Set<Section> expanded = new HashSet<>();
		expanded.add(section);
		section.getContents().stream() //
				.filter(Section.class::isInstance) //
				.map(Section.class::cast) //
				.map(CxxIntegrityTest::expand) //
				.flatMap(Collection::stream) //
				.forEach(expanded::add);
		return expanded;
	}}

}}
", template.package, template.url)
}

fn sub_tests(section: &Section) -> String {
    let content = &section.content;
    content
        .into_iter()
        .map(&expand_sections)
        .flat_map(Vec::into_iter)
        .map(|section| single_test(&section))
        .collect::<Vec<String>>()
        .join(" ")
}

fn expand_sections(section: &Section) -> Vec<&Section> {
    let mut sections = Vec::new();
    let content = &section.content;
    sections.push(section);
    content
        .into_iter()
        .map(|item| expand_sections(&item))
        .flat_map(Vec::into_iter)
        .for_each(|item| sections.push(item));
    sections
}

fn content(section: &Section, template: &TestConfiguration) -> String {
    format!(
        "/*******************************************************************************
 * Copyright (c) ArSysOp 2018-2022
 * 
 * RGM Sources are publicly available only for 
 * informational, review, analysis and consulting purposes.
 * 
 * Definitions, terms and conditions for using RGM Sources are stated by ArSysOp Source License version 1.0.
 * See http://arsysop.ru/licenses/rgm/ArSysOpSourceLicense-1.0.txt
 * 
 * RGM Sources are provided on \"as is\" basis. 
 * ArSysOp is not responsible for any damages, losses, legal prosecution 
 * or other consequences of any sort that using RGM Sources can cause to you 
 * (as an individual or Legal Entity), even if aware of such consequences.
 * 
*******************************************************************************/
package {};
    
import static org.junit.Assert.assertEquals;
import org.junit.Test;

import ru.arsysop.loft.rgm.spec.model.api.Section;

public final class {} extends CxxIntegrityTest {{
            
    @Test
    public void directContents() {{
        assertEquals({}, sections().stream() //
                .filter(section -> section.getId().equals(\"{}\")) //$NON-NLS-1$
                .flatMap(section -> section.getContents().stream()) //
                .filter(Section.class::isInstance) //
                .count());
    }}

{}
}}\n",
        template.package,
        class_name(section),
        section.content.len(),
        section.id,
        sub_tests(section)
    )
}
