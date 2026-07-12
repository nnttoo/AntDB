// Haryanto 12 July 2026

import * as fs from 'fs';
import * as path from 'path';

function formatThousandSeparator(value: string): string { 
    const num = Number(value.replace(/,/g, ''));  
    if (!isNaN(num) && value.trim() !== '') { 
        // Use Intl.NumberFormat to handle thousand separators and decimals accurately
        return new Intl.NumberFormat('en-US').format(num);
    }
    return value;
}

function convertCsvToMarkdown(csvFilePath: string, mdFilePath: string): void {
    try {
        const csvContent = fs.readFileSync(csvFilePath, 'utf-8');
        const lines = csvContent.split(/\r?\n/).map(line => line.trim()).filter(line => line.length > 0);

        if (lines.length === 0) {
            console.error('The CSV file is empty.');
            return;
        }

        const tableRows: string[][] = [];
        let headers: string[] = [];

        lines.forEach((line, index) => { 
            const columns = line.split(/,(?=(?:(?:[^"]*"){2})*[^"]*$)/).map(col => {
                return col.replace(/^"|"$/g, '').trim();
            });

            if (index === 0) {
                headers = columns;
            } else {
                if (columns[0] === headers[0]) {
                    return;  
                }
 
                if (columns[1]) {
                    columns[1] = formatThousandSeparator(columns[1]);
                }

                tableRows.push(columns);
            }
        });
 
        let markdown = '| ' + headers.join(' | ') + ' |\n';
        markdown += '| ' + headers.map(() => '---').join(' | ') + ' |\n';

        tableRows.forEach(row => {
            markdown += '| ' + row.join(' | ') + ' |\n';
        });

        let date = new Date();
        let content = `## Benchmark Result : \n\n 
        \nDate : ${date.toLocaleDateString() }  - ${date.toLocaleTimeString()}
        \n\n${markdown}`

        fs.writeFileSync(mdFilePath, content, 'utf-8');
        console.log(`Successfully generated Markdown table at: ${mdFilePath}`);

    } catch (error) {
        console.error('An error occurred during processing:', error);
    }
}

// Execution paths
const csvPath = path.join(__dirname, '../docker_working_dir/benchmark_results.csv');
const mdPath = path.join(__dirname, '../benchmark_results.md');

convertCsvToMarkdown(csvPath, mdPath);